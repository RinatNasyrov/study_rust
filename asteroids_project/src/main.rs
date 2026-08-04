use asteroids_project::State;
use macroquad::prelude::*;

// Точка входа в приложение. Макрос позволяет сделать функцию main асинхронной,
// а также иницилизирует окно.
#[macroquad::main("Asteroids")]
async fn main() {
    // Инициализирум состояние наший игры по умолчанию.
    let mut state = State::default();

    // Запускаем игровой цикл.
    loop {
        // Очищаем фон тёмно-серым цветом.
        clear_background(DARKGRAY);

        // Обновляем состояние игры.
        state.update();

        // Отображаем игру в окне.
        state.draw();

        // Ожидаем возможности заняться следующим кадром.
        next_frame().await;
    }
}
