// Подключаем модуль по имени файла
mod entities;
// Реэкспортируем содержание модуля, чтобы здесь и
// снаружи этого библиотечного крейта можно было
// использовать содержимое модуля entities как если бы оно
// было объявлено в lib.rs (в корне библиотечного крейта)
pub use entities::*;

use macroquad::prelude::*;

/// Состояние приложения.
pub struct State {
    /// Рекорное время.
    best_time: f64,
    /// Состояние игрового процесса.
    game: Option<Game>,
}

/// Логика создания состояния приложения.
impl Default for State {
    fn default() -> Self {
        Self {
            best_time: 0.0,
            game: None, // Изначально находимся в меню.
        }
    }
}

impl State {
    /// Логика обновления приложения.
    pub fn update(&mut self) {
        // Если нажат Enter - запускаем игру.
        if self.game.is_none() && is_key_pressed(KeyCode::Enter) {
            let game = Game::default(); // Создаём новое состояние игрового процесса.
            self.game = Some(game); // Запоминаем его.
            return;
        }

        // Если мы в игре - обновляем её состояние.
        let finished = self.game
      .as_mut(). // получаем уникальную (мутабельную) ссылку на содержимое Option, если оно есть.
      and_then(|game| { // Если получили, то выполняем функтор,
        game.update() // который обновляет состояние игры.
      });

        // Если игра завершена - то получим время, которое игроку удалось продержаться.
        if let Some(new_time) = finished {
            self.game = None; // Завершаем игру.
            if new_time > self.best_time {
                // Если новое время дольше рекордного,
                self.best_time = new_time; // то обновляем рекорд.
            }
        }
    }

    /// Отображение приложения.
    pub fn draw(&self) {
        // Если игра запущена - отображаем её,
        if let Some(game) = &self.game {
            game.draw(self.best_time)
        } else {
            // иначе, рисуем меню.
            Self::draw_menu(self.best_time)
        }
    }

    /// Отображение меню
    fn draw_menu(best_time: f64) {
        Self::draw_text(
            &format!("Press Enter to start game.\nBest time: {:.2}", best_time),
            40.0,
        );
    }

    fn draw_text(text: &str, font_size: f32) {
        // Вычисляем, какой размер занимает текст на экране.
        let text_size = measure_text(text, None, font_size as _, 1.0);

        // Располагаем текст по центру.
        let text_pos = (
            (screen_width() - text_size.width) / 2.0,
            (screen_height() - text_size.height) / 2.0,
        );

        // Отображаем текст
        draw_text(text, text_pos.0, text_pos.1, font_size, BLACK);
    }
}

/// Состояние игрового процесса.
struct Game {
    /// Время, когда игра запустилась.
    start_time: f64,
    /// Время предыдущего обновления состояния игры.
    last_update: f64,
    /// Корабль игрока.
    ship: Ship,
    /// Таймер появления астероидов.
    asteroid_timer: f64,
    /// Вектор астероидов.
    asteroids: Vec<Asteroid>,
}

impl Default for Game {
    /// Логика создания новой игры.
    fn default() -> Self {
        let time = get_time(); // Текущее время со старта приложения.
        Self {
            start_time: time,
            last_update: time,
            ship: Ship::default(),
            asteroid_timer: 0.0,
            asteroids: Vec::with_capacity(100), // Создаём пустой вектор,
                                                // способный вместить в себя до 100 астероидов без дополнительных аллокаций.
        }
    }
}

impl Game {
    /// Логика обновления игрового процесса.
    pub fn update(&mut self) -> Option<f64> {
        if is_key_pressed(KeyCode::Escape) {
            // Если нажат Escape - выходим в меню.
            return Some(get_time() - self.start_time);
        }

        let elapsed_time = self.elapsed_time(); // Время, прошедшее с предыдущего кадра.

        self.ship.update(elapsed_time); // Обновляем состояние корабля.

        self.last_update = get_time(); // Запоминаем время завершения обновления кадра.

        // Менеджмент астероидов
        self.asteroid_timer += elapsed_time; // Обновляем таймер появления астероидов.
        if self.asteroid_timer > 0.5 {
            // Если астероид не появлялся уже полсекунды,
            self.asteroid_timer = 0.0; // сбрасываем таймер
            self.asteroids.push(Asteroid::default()); // и добавляем новый астероид.
        }

        // Забываем астероиды, вышедшие за пределы экрана.
        self.asteroids.retain(|asteroid| !asteroid.out_of_bounds());

        // Обновляем состояние астероиндов.
        for asteroid in &mut self.asteroids {
            asteroid.update(elapsed_time, self.ship.vertical_speed());

            if self
                .ship
                .is_collapse(asteroid.position(), asteroid.radius())
            {
                // Если астероид столкнулся с кораблём, то завершаем игру.
                return Some(self.game_time());
            }
        }
        None // Игра продолжается.
    }

    /// Время, прошедшее с последнего обновления.
    fn elapsed_time(&self) -> f64 {
        get_time() - self.last_update
    }

    /// Отображаем игру.
    pub fn draw(&self, best_time: f64) {
        self.draw_time(best_time); // Отображаем текст с лучшим и текущим временем.
        self.ship.draw(); // Отображаем корабль.

        // Отображаем астероиды.
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }

    /// Отображаем текст с лучшим и текущим временем.
    fn draw_time(&self, best_time: f64) {
        let font_size = 24.0;
        let text = format!("Best time: {:.2}", best_time);
        let text_size = measure_text(&text, None, font_size as _, 1.0);
        draw_text(&text, 0.0, screen_height(), font_size, BLACK);

        let time = self.game_time();
        let text = format!("Your time: {:.2}", time);

        // Если текущее время лучше рекордного, отображаем его зелёным цветом.
        let color = if time > best_time { GREEN } else { BLACK };

        draw_text(
            &text,
            0.0,
            screen_height() - text_size.height,
            font_size,
            color,
        );
    }

    /// Время в текущей игре.
    fn game_time(&self) -> f64 {
        get_time() - self.start_time
    }
}
