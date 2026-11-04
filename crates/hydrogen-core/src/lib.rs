//! Cœur sans I/O du moteur déclaratif Hydrogen.
//! Contient les primitives d'identifiants, de valeurs scalaires, d'état d'entités,
//! d'événements et d'erreurs typées.

pub mod entity;
pub mod error;
pub mod event;
pub mod identifier;
pub mod value;

pub use entity::{ComponentFields, EntityState, FieldAddress, WorldState};
pub use error::CoreError;
pub use event::{EventBus, RuntimeEvent};
pub use identifier::{validate_identifier_str, Identifier};
pub use value::{ScalarValue, ValueKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_accepts_valid_snake_case_and_rejects_invalid_inputs() {
        assert!(Identifier::new("player_1").is_ok());
        assert!(Identifier::new("health").is_ok());

        assert_eq!(
            Identifier::new(""),
            Err(CoreError::InvalidIdentifierLength { length: 0 })
        );
        assert!(matches!(
            Identifier::new("1_invalid"),
            Err(CoreError::InvalidIdentifierFormat { .. })
        ));
        assert!(matches!(
            Identifier::new("InvalidCamel"),
            Err(CoreError::InvalidIdentifierFormat { .. })
        ));
    }

    #[test]
    fn scalar_value_rejects_non_finite_floats() {
        assert_eq!(ScalarValue::try_float(42.5), Ok(ScalarValue::Float(42.5)));
        assert_eq!(
            ScalarValue::try_float(f64::NAN),
            Err(CoreError::NonFiniteFloat)
        );
        assert_eq!(
            ScalarValue::try_float(f64::INFINITY),
            Err(CoreError::NonFiniteFloat)
        );
    }

    #[test]
    fn world_state_reads_and_writes_entity_component_fields() -> Result<(), CoreError> {
        let entity_id = Identifier::new("hero")?;
        let component_id = Identifier::new("stats")?;
        let field_id = Identifier::new("hp")?;

        let mut entity = EntityState::new(entity_id.clone());
        let mut fields = ComponentFields::new();
        fields.insert(field_id.clone(), ScalarValue::Int(100));
        entity.components.insert(component_id.clone(), fields);

        let mut world = WorldState::new();
        world.insert_entity(entity);

        let address = FieldAddress {
            entity_id,
            component_id,
            field_id,
        };

        assert_eq!(
            world.read_entity_field(&address),
            Some(&ScalarValue::Int(100))
        );

        world.write_entity_field(&address, ScalarValue::Int(80))?;

        assert_eq!(
            world.read_entity_field(&address),
            Some(&ScalarValue::Int(80))
        );
        Ok(())
    }

    #[test]
    fn event_bus_preserves_fifo_ordering() -> Result<(), CoreError> {
        let mut bus = EventBus::new();
        let first = RuntimeEvent::new(Identifier::new("tick")?);
        let second = RuntimeEvent::new(Identifier::new("damage_applied")?);

        bus.publish(first.clone());
        bus.publish(second.clone());

        assert_eq!(bus.len(), 2);
        assert_eq!(bus.pop_next(), Some(first));
        assert_eq!(bus.pop_next(), Some(second));
        assert!(bus.is_empty());
        Ok(())
    }
}
