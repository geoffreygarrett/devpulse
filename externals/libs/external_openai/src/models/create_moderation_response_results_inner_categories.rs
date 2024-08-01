/*
 * OpenAI API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// CreateModerationResponseResultsInnerCategories : A list of the categories, and whether they are flagged or not.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct CreateModerationResponseResultsInnerCategories {
    /// Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harassment.
    #[serde(rename = "hate")]
    pub hate: bool,
    /// Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste.
    #[serde(rename = "hate/threatening")]
    pub hate_slash_threatening: bool,
    /// Content that expresses, incites, or promotes harassing language towards any target.
    #[serde(rename = "harassment")]
    pub harassment: bool,
    /// Harassment content that also includes violence or serious harm towards any target.
    #[serde(rename = "harassment/threatening")]
    pub harassment_slash_threatening: bool,
    /// Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm/intent")]
    pub self_harm_slash_intent: bool,
    /// Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts.
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_slash_instructions: bool,
    /// Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
    #[serde(rename = "sexual")]
    pub sexual: bool,
    /// Sexual content that includes an individual who is under 18 years old.
    #[serde(rename = "sexual/minors")]
    pub sexual_slash_minors: bool,
    /// Content that depicts death, violence, or physical injury.
    #[serde(rename = "violence")]
    pub violence: bool,
    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[serde(rename = "violence/graphic")]
    pub violence_slash_graphic: bool,
}

impl CreateModerationResponseResultsInnerCategories {
    /// A list of the categories, and whether they are flagged or not.
    pub fn new(
        hate: bool, hate_slash_threatening: bool, harassment: bool,
        harassment_slash_threatening: bool, self_harm: bool, self_harm_slash_intent: bool,
        self_harm_slash_instructions: bool, sexual: bool, sexual_slash_minors: bool,
        violence: bool, violence_slash_graphic: bool,
    ) -> CreateModerationResponseResultsInnerCategories {
        CreateModerationResponseResultsInnerCategories {
            hate,
            hate_slash_threatening,
            harassment,
            harassment_slash_threatening,
            self_harm,
            self_harm_slash_intent,
            self_harm_slash_instructions,
            sexual,
            sexual_slash_minors,
            violence,
            violence_slash_graphic,
        }
    }

    pub fn builder() -> CreateModerationResponseResultsInnerCategoriesBuilder {
        CreateModerationResponseResultsInnerCategoriesBuilder::create_empty()
    }
}