pub mod vao_builder;
pub mod camera;
pub mod shader_program;
pub mod ecs;

#[cfg(test)]
mod tests {
    use super::vao_builder::*;

    #[test]
    fn can_create_vao_builder() {
        let builder = VaoBuilder::new();
        assert_eq!(2 + 2, 4);
    }
}
