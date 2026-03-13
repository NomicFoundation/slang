use crate::ir::{IrModel, IrModelMutator, ModelWithBuilder};

pub(super) fn build_from(cst_model: &IrModel) -> ModelWithBuilder {
    let mut mutator = IrModelMutator::create_from(cst_model);

    // remove fields from sequences that contain redundant terminal nodes
    for (sequence_id, sequence) in &cst_model.sequences {
        if sequence.multiple_operators {
            // don't remove terminals if the sequence is modelling a precedence
            // expression with multiple variant operators
            continue;
        }
        for field in &sequence.fields {
            if !field.is_optional
                && field.r#type.is_terminal()
                && cst_model.terminals[field.r#type.as_identifier()]
            {
                mutator.remove_sequence_field(sequence_id, &field.label);
            }
        }
    }

    mutator.into()
}
