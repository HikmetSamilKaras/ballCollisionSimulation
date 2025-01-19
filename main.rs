use macroquad::prelude::*;

struct Ball {
    position: [f32;2],
    radius: f32,
    velocity: [f32;2],
    acceleration: [f32;2],
    color: Color,
}

#[macroquad::main("BallSimulation")]
async fn main() {
    let mut vec: Vec<Ball> = Vec::new();

    loop {
        if is_key_pressed(KeyCode::R) {
            vec.clear();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mut flag = true;
            for ball in &mut vec {
                if (ball.position[0] - mouse_position().0).powi(2) + (ball.position[1] - mouse_position().1).powi(2) <= 4.0*ball.radius.powi(2) {
                    ball.velocity[0] += ball.position[0] - mouse_position().0;
                    ball.velocity[1] += ball.position[1] - mouse_position().1;
                    flag = false;
                }
            }   

            if flag { 
                vec.push(Ball {
                    position: [mouse_position().0, mouse_position().1],
                    radius: 15.0,
                    acceleration: [0.0, 10.0],
                    velocity: [0.0, 0.0],
                    color: RED,
                });
            }    
        }

        for i in 0..vec.len() {
            for j in i + 1..vec.len(){
                if (vec[i].position[0] - vec[j].position[0]).powi(2) + (vec[i].position[1] - vec[j].position[1]).powi(2) <= (vec[i].radius + vec[j].radius).powi(2) {
                    // Calculate the distance between the centers
                    let dx = vec[i].position[0] - vec[j].position[0];
                    let dy = vec[i].position[1] - vec[j].position[1];
                    let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                    // Calculate overlap
                    let overlap = (vec[i].radius + vec[j].radius) - distance;

                    // Normalize the displacement direction
                    let nx = dx / distance;
                    let ny = dy / distance;

                    // Push objects apart to resolve overlap
                    vec[i].position[0] += nx * overlap / 2.0;
                    vec[i].position[1] += ny * overlap / 2.0;
                    vec[j].position[0] -= nx * overlap / 2.0;
                    vec[j].position[1] -= ny * overlap / 2.0;

                    // Swap velocities
                    let temp = vec[i].velocity;
                    vec[i].velocity = vec[j].velocity;
                    vec[j].velocity = temp;
                }

            }
        }

        let totalEnergy: f32 = vec.iter().map(|ball| 0.5 * (ball.velocity[0].powi(2) + ball.velocity[1].powi(2)) + (screen_height()-ball.position[1])*10.0).sum();

        for ball in &mut vec {
            ball.velocity[0] += ball.acceleration[0] * get_frame_time();
            ball.velocity[1] += ball.acceleration[1] * get_frame_time();

            ball.position[0] += ball.velocity[0] * get_frame_time();
            ball.position[1] += ball.velocity[1] * get_frame_time();

            if ball.position[1] + ball.radius > screen_height() {
                ball.velocity[1] = -ball.velocity[1];
                ball.position[1] = screen_height() - ball.radius;
            }

            if ball.position[0] + ball.radius > screen_width() {
                ball.velocity[0] = -ball.velocity[0];
                ball.position[0] = screen_width() - ball.radius;
            }

            if ball.position[0] - ball.radius < 0.0 {
                ball.velocity[0] = -ball.velocity[0];
                ball.position[0] = ball.radius;
            }

            if ball.position[1] - ball.radius < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
                ball.position[1] = ball.radius;
            }
        }

        //println!("Total Energy: {}", totalEnergy);
        clear_background(WHITE);

        for ball in &vec {
            draw_circle(ball.position[0], ball.position[1], ball.radius, ball.color);
        }

        next_frame().await  
    }
}