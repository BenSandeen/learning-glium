mod vert_shader {
    let vert_shader = r#"
    #version 330

    in vec2 Position;

    void main() {
        glPosition = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
}