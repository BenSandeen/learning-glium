pub mod frag_shader {
    pub struct Shader {
    shader: { let frag_shader = r#"
    #version 330

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
    }
}