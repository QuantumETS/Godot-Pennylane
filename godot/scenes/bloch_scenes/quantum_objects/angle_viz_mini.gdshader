shader_type spatial;
render_mode cull_disabled;
uniform vec4 color : source_color = vec4(1.0);
uniform float diameter = 1.0; // Circle Diameter
uniform float thickness = 0.05; // Outline thickness
uniform float frequency = 10.0; // Controls the number of dots
uniform float phase = 0.0; // Controls the rotation of the circle
uniform float theta_start = 0.0; // Start angle in radians
uniform float theta_end = 3.14; // End angle in radians


void fragment() {
  vec2 pos = UV - vec2(0.5);
  float outer_radius = diameter / 2.0;
  float inner_radius = outer_radius - thickness;
  float outer_circle = step(length(pos), outer_radius);
  float inner_circle = step(length(pos), inner_radius);
  EMISSION = vec3(1.0,1.0,1.0);

  float angle = atan(pos.y, pos.x);
  if (angle < 0.0) {
    angle += 2.0 * PI;
  }
  float angle_mask = step(theta_start, angle) * step(angle, theta_end);
  float wave = 0.5 * sin(frequency * angle + phase) + 0.5;
  

  float ring = (outer_circle - inner_circle) * angle_mask;
  //ring *= step(0.5, wave);  
	ALBEDO = color.rgb;
	ALPHA = ring * color.a;
}