use std::num::NonZeroU32;
use std::path::Path;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;

use crate::types::LocalLlmModel;

/// Moteur LLM local basé sur llama.cpp
pub struct LocalLlmEngine {
    model: LlamaModel,
    backend: LlamaBackend,
    model_type: LocalLlmModel,
}

impl LocalLlmEngine {
    pub fn new(model_path: &Path, model_type: LocalLlmModel) -> Result<Self, String> {
        log::info!("Loading LLM model {:?} from {:?}", model_type.display_name(), model_path);

        if !model_path.exists() {
            return Err(format!("Model file not found: {:?}", model_path));
        }

        // Initialiser le backend llama.cpp
        let backend = LlamaBackend::init()
            .map_err(|e| format!("Failed to initialize llama backend: {}", e))?;

        // Paramètres du modèle
        let model_params = LlamaModelParams::default();

        // Charger le modèle
        let model = LlamaModel::load_from_file(&backend, model_path, &model_params)
            .map_err(|e| format!("Failed to load LLM model: {}", e))?;

        log::info!("LLM model loaded successfully: {}", model_type.display_name());

        Ok(Self {
            model,
            backend,
            model_type,
        })
    }

    /// Génère un résumé du texte donné
    pub fn summarize(&self, text: &str) -> Result<String, String> {
        let start_time = std::time::Instant::now();

        // Tronquer le texte si trop long (garder ~1500 chars pour laisser de la place au prompt et à la réponse)
        let max_text_len = 1500;
        let truncated_text = if text.len() > max_text_len {
            log::info!("Truncating text from {} to {} chars", text.len(), max_text_len);
            &text[..max_text_len]
        } else {
            text
        };

        // Créer le contexte d'inférence (contexte réduit pour les petits modèles)
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(2048));

        let mut ctx = self.model
            .new_context(&self.backend, ctx_params)
            .map_err(|e| format!("Failed to create context: {}", e))?;

        // Construire le prompt selon le modèle (format ChatML pour SmolLM2 et Qwen2.5)
        let instruction = "Tu es un assistant qui fait des résumés concis en français. Résume le texte suivant en 2-3 phrases maximum. Retourne uniquement le résumé, rien d'autre.";
        let prompt = self.model_type.format_prompt(instruction, truncated_text);

        log::info!("LLM prompt ({} chars): {}...", prompt.len(), &prompt[..prompt.len().min(200)]);

        // Tokenizer le prompt (pas de BOS car déjà dans le format ChatML)
        let tokens = self.model
            .str_to_token(&prompt, AddBos::Never)
            .map_err(|e| format!("Failed to tokenize: {}", e))?;

        log::info!("Tokenized to {} tokens", tokens.len());

        if tokens.is_empty() {
            return Err("No tokens generated from prompt".to_string());
        }

        // Créer un batch pour l'inférence (taille suffisante pour le prompt)
        let batch_size = (tokens.len() + 200).max(512);
        let mut batch = LlamaBatch::new(batch_size, 1);

        // Ajouter les tokens du prompt au batch
        for (i, token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch.add(*token, i as i32, &[0], is_last)
                .map_err(|e| format!("Failed to add token to batch: {}", e))?;
        }

        // Décoder le prompt
        ctx.decode(&mut batch)
            .map_err(|e| format!("Failed to decode prompt: {}", e))?;

        // Créer le sampler greedy pour la génération
        let mut sampler = LlamaSampler::greedy();

        // Générer la réponse (limite réduite pour résumés courts)
        let mut output_tokens = Vec::new();
        let max_tokens = 150;
        let mut n_cur = tokens.len();

        log::info!("Starting generation, max_tokens={}", max_tokens);

        for i in 0..max_tokens {
            // Échantillonner le prochain token
            let new_token_id = sampler.sample(&ctx, batch.n_tokens() - 1);

            // Accepter le token
            sampler.accept(new_token_id);

            // Vérifier si c'est un token de fin
            if self.model.is_eog_token(new_token_id) {
                log::info!("EOG token reached at position {}", i);
                break;
            }

            output_tokens.push(new_token_id);

            // Préparer le prochain batch
            batch.clear();
            batch.add(new_token_id, n_cur as i32, &[0], true)
                .map_err(|e| format!("Failed to add token: {}", e))?;

            n_cur += 1;

            // Décoder
            ctx.decode(&mut batch)
                .map_err(|e| format!("Failed to decode: {}", e))?;
        }

        log::info!("Generated {} tokens", output_tokens.len());

        // Convertir les tokens en texte
        let output_text: String = output_tokens
            .iter()
            .filter_map(|t| {
                self.model.token_to_piece_bytes(*t, 128, false, None)
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok())
            })
            .collect();

        let processing_time = start_time.elapsed().as_millis();
        log::info!("Local LLM summarization completed in {}ms, output: {} chars", processing_time, output_text.len());

        Ok(output_text.trim().to_string())
    }

    pub fn model_type(&self) -> LocalLlmModel {
        self.model_type
    }

    pub fn display_name(&self) -> String {
        format!("Local LLM ({})", self.model_type.display_name())
    }
}

unsafe impl Send for LocalLlmEngine {}
unsafe impl Sync for LocalLlmEngine {}
