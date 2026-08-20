use macroquad::prelude::*;
use my_game::Shape;

#[macroquad::main("My game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const SHOOTING_COOLDOWN: f64 = 1.0; // Секунды
    let mut last_shot_time = get_time();

    // Задаем сид генератора сч
    rand::srand(miniquad::date::now() as u64);

    // Храним фигуры
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };
    let mut bullets: Vec<Shape> = vec![];

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

            // Квадраты генерируем
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    collided: false,
                });
            }

            // Квадраты удаляем
            squares.retain(|square| square.y < screen_height() + square.size);

            // Квадраты двигаем
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Делаем снаряды
            if is_key_pressed(KeyCode::Space) {
                if get_time() - last_shot_time > SHOOTING_COOLDOWN {
                    bullets.push(Shape {
                        x: circle.x,
                        y: circle.y,
                        speed: circle.speed * 2.0,
                        size: 5.0,
                        collided: false,
                    });
                    last_shot_time = get_time();
                }
            }

            // Снаряды двигаем
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // Снаряды удалим
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

            // Проставим коллизии
            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                    }
                }
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
                bullets.clear();
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                gameover = false;
            }
        }

        // Рисуем кароче независимо ни от чего
        // Круг рисуем
        draw_circle(circle.x, circle.y, 16.0, YELLOW);

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

        // Снаряды рисуем
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }

        next_frame().await
    }
}
