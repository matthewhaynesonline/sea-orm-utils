#[macro_export]
macro_rules! impl_related_to {
    ($target_table_name:ident, $target_model_name:ident) => {
        impl Related<super::$target_table_name::Entity> for Entity {
            fn to() -> RelationDef {
                Relation::$target_model_name.def()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_related_to_via {
    ($source_model_name:ident, $via_table_name:ident, $target_table_name:ident, $target_model_name:ident) => {
        impl Related<super::$target_table_name::Entity> for Entity {
            fn to() -> RelationDef {
                super::$via_table_name::Relation::$target_model_name.def()
            }

            fn via() -> Option<RelationDef> {
                Some(
                    super::$via_table_name::Relation::$source_model_name
                        .def()
                        .rev(),
                )
            }
        }
    };
}
