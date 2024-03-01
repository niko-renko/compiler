use super::*;

pub struct Peephole {
    this_id: Option<usize>,
}

impl Peephole {
    pub fn from(this_id: Option<usize>) -> Self {
        Peephole { this_id }
    }
}

impl Update for Peephole {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        if self.this_id.is_none() {
            return Ok(());
        }

        let this_id = self.this_id.unwrap();

        for label in &*cfg {
            let block = cfg.get_block_mut(label);

            for (_, instruction) in block.get_instructions_mut().iter_mut() {
                let op = match instruction {
                    Instruction::Op(op) => op,
                    _ => continue,
                };

                if matches!(op.get_operator(), Operator::And) {
                    continue;
                }

                let mut has_one = false;
                let mut has_this = false;

                for used in op.used() {
                    match used {
                        PlaceValue::Place(Place::Named(named)) => {
                            if named.get_id() == this_id {
                                has_this = true
                            }
                        }
                        PlaceValue::Value(value) => {
                            if value.get_value() == 1 {
                                has_one = true;
                            }
                        }
                        _ => {}
                    }
                }

                if has_one && has_this {
                    *instruction = Alias::from(Value::from_raw(0).into()).into();
                    continue;
                }
            }
        }

        Ok(())
    }
}
