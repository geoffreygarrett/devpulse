// use std::collections::HashSet;
// use crate::config::models::*;
//
// mod models;
//
// use models as fga_models;
//
//
// impl From<&RelationConfig> for fga_models::InternalRelation {
//     fn from(config: &RelationConfig) -> Self {
//         match config {
//             RelationConfig::Relation(relation) => match relation {
//                 Relation::Direct(direct_relations) => {
//                     let set: HashSet<String> = direct_relations.iter().map(|r| match r {
//                         DirectRelation::TupleString(s) => s.clone(),
//                         DirectRelation::TypeRelation { type_, relation } => format!("{}#{}", type_, relation),
//                     }).collect();
//                     fga_models::InternalRelation::Direct(set)
//                 },
//                 Relation::Indirect(indirect_relation) => match indirect_relation {
//                     fga_modelsIndirectRelation::Internal(s) => InternalRelation::ComputedUserset {
//                         relation: s.clone(),
//                     },
//                     IndirectRelation::External { relation, from } => {
//                         InternalRelation::ComputedUserset {
//                             relation: format!("{}#{}", from, relation),
//                         }
//                     }
//                 },
//             },
//             RelationConfig::Conditional(conditional) => {
//                 if let Some(union) = &conditional.union {
//                     InternalRelation::Union(union.iter().map(Into::into).collect())
//                 } else if let Some(intersection) = &conditional.intersection {
//                     InternalRelation::Intersection(intersection.iter().map(Into::into).collect())
//                 } else if let Some(exclusion) = &conditional.exclusion {
//                     InternalRelation::Exclusion(exclusion.iter().map(Into::into).collect())
//                 } else {
//                     panic!("Invalid conditional relation")
//                 }
//             }
//         }
//     }
// }
//
// impl From<&RelationVariant> for InternalRelation {
//     fn from(variant: &RelationVariant) -> Self {
//         match variant {
//             RelationVariant::Relation(relation) => InternalRelation::from(&RelationConfig::Relation(relation.clone())),
//             RelationVariant::Conditional(conditional) => InternalRelation::from(&RelationConfig::Conditional(*conditional.clone())),
//         }
//     }
// }
//
// impl From<&TypeConfig> for InternalType {
//     fn from(config: &TypeConfig) -> Self {
//         let relations = if let Some(relation_configs) = &config.relations {
//             relation_configs.iter().map(|(relation_name, relation_config)| {
//                 (relation_name.clone(), InternalRelation::from(relation_config))
//             }).collect()
//         } else {
//             HashMap::new()
//         };
//         InternalType { relations }
//     }
// }
//
// impl From<&Configuration> for Schema {
//     fn from(config: &Configuration) -> Self {
//         let types = config.types.iter().map(|(type_name, type_config)| {
//             (type_name.clone(), InternalType::from(type_config))
//         }).collect();
//         Schema {
//             schema_version: config.model.schema.clone(),
//             types,
//         }
//     }
// }
//
// fn main() {
//     // Example input configuration (you can replace this with actual data)
//     let config = Configuration {
//         model: ModelConfig {
//             schema: "1.1".to_string(),
//         },
//         types: HashMap::new(), // Add your type configs here
//     };
//
//     // Convert to internal model
//     let internal_model: Schema = Schema::from(&config);
//
//     // Print the internal model to verify the conversion
//     println!("{:#?}", internal_model);
// }

pub mod models;
mod conversion;