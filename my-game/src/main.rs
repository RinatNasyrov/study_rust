use macroquad::prelude::*;
use my_game::Shape;

#[macroquad::main("My game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    // Задаем сид генератора сч
    rand::srand(miniquad::date::now() as u64);

    // Храним фигуры
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    // Состояние игры
    let mut gameover = false;

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();

        // Круг двигаем
        if !gameover {
            if is_key_down(KeyCode::Right) {
                circle.x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= MOVEMENT_SPEED * delta_time;
            }

            // Круг ограничиваем в движении
            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            // Круг рисуем
            draw_circle(circle.x, circle.y, 16.0, YELLOW);

            // Квадраты генерируем
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                });
            }

            // Квадраты удаляем
            squares.retain(|square| square.y < screen_height() + square.size);

            // Квадраты двигаем
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Квадраты рисуем
            for square in &squares {
                draw_rectangle(
                    square.x - square.size / 2.0,
                    square.y - square.size / 2.0,
                    square.size,
                    square.size,
                    GREEN,
                );
            }

            // Закончим игру
            if squares.iter().any(|square| circle.collides_with(square)) {
                gameover = true;
            }
        } else {
            // Сообщение о конце игры
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );

            // Продолжаем игру
            if is_key_pressed(KeyCode::Space) {
                squares.clear();
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                gameover = false;
            }
        }

        next_frame().await
    }
}
