use macroquad::prelude::*;
use macroquad::rand::RandomRange;

/// Состояние корабля.
pub struct Ship {
    /// Положение по горизонтали.
    position: f32,
    /// Скорость по горизонтали.
    speed: f32,
    /// Скорость по вертикали (с которой, относительно корабля, движутся астероиды)
    vertical_speed: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: screen_width() / 2.0, // Изначально корабль находится по центру окна.
            speed: 0.0,
            vertical_speed: 100.0,
        }
    }
}

impl Ship {
    // Параметры корабля.
    const SHIP_WIDTH: f32 = 25.0;
    const SHIP_HEIGHT: f32 = 50.0;
    const SHIP_OFFSET: f32 = 30.0;

    pub fn vertical_speed(&self) -> f32 {
        self.vertical_speed
    }

    /// Логика обновления корабля.
    pub fn update(&mut self, elapsed_time: f64) {
        const ACCELERATION: f32 = 200.0;
        const VERTICAL_ACCELERATION: f32 = 50.0;
        const DECELERATION: f32 = 180.0;
        let elapsed_time = elapsed_time as f32;

        // Замедляем корабль по горизонтали.
        self.speed /= (DECELERATION * elapsed_time).exp();

        // Если нажата А, то ускоряем корабль влево.
        if is_key_down(KeyCode::A) {
            self.speed -= ACCELERATION * elapsed_time;
        }

        // Если нажата D, то ускоряем корабль вправо.
        if is_key_down(KeyCode::D) {
            self.speed += ACCELERATION * elapsed_time;
        }

        // Перемещаем корабль.
        self.position += self.speed;

        // Не даём кораблю выйти за пределы окна.
        self.position = self.position.clamp(
            Self::SHIP_WIDTH / 2.0,
            screen_width() - Self::SHIP_WIDTH / 2.0,
        );

        // Ускоряем корабль по вертикали для повышения сложности игры со временем.
        self.vertical_speed += VERTICAL_ACCELERATION * elapsed_time;
    }

    /// Отображаем корабль.
    pub fn draw(&self) {
        // Вычисляем точки треугольника.
        let top = Vec2::new(
            self.position,
            screen_height() - Self::SHIP_HEIGHT / 2.0 - Self::SHIP_OFFSET,
        );
        let left = Vec2::new(
            self.position - Self::SHIP_WIDTH / 2.0,
            screen_height() - Self::SHIP_OFFSET,
        );
        let right = Vec2::new(
            self.position + Self::SHIP_WIDTH / 2.0,
            screen_height() - Self::SHIP_OFFSET,
        );

        // Отображаем треугольник.
        draw_triangle(top, right, left, WHITE)
    }

    /// Столкнулся ли корабль с кругом с центром в `point` и радиусом `radius`.
    pub fn is_collapse(&self, point: Vec2, radius: f32) -> bool {
        // Вычисляем приблизительный радиус корабля.
        let ship_radius = (Self::SHIP_WIDTH + Self::SHIP_HEIGHT) / 4.0;

        // Вычисляем положение центра корабля.
        let ship_center = Vec2::new(self.position, screen_height() - Self::SHIP_OFFSET);

        // Проверяем, не пересекаются ли радиусы корабля и круга.
        (point - ship_center).length() < radius + ship_radius
    }
}

/// Состояние астероида.
pub struct Asteroid {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

impl Default for Asteroid {
    fn default() -> Self {
        // Располагаем астероид случайно, немного выше видимого экрана.
        let x = f32::gen_range(0.0, screen_width());
        let y = -2.0 * Self::MAX_RADIUS;

        // Задаём случайную скорость астероиду.
        let speed_x = f32::gen_range(0.0, Self::MAX_SPEED);
        let speed_y = f32::gen_range(0.0, Self::MAX_SPEED);

        Self {
            position: Vec2::new(x, y),
            speed: Vec2::new(speed_x, speed_y),
            radius: f32::gen_range(Self::MIN_RADIUS, Self::MAX_RADIUS),
        }
    }
}

impl Asteroid {
    // Параметры астероидов
    const MIN_RADIUS: f32 = 25.0;
    const MAX_RADIUS: f32 = 100.0;
    const MAX_SPEED: f32 = 200.0;

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn position(&self) -> Vec2 {
        // Vec2 реализует трейт Copy поэтому
        // по идее не нужно делать явную копию
        self.position
    }

    /// Проверка выхода астероида далеко за границы экрана.
    pub fn out_of_bounds(&self) -> bool {
        let (x, y) = (self.position.x, self.position.y);
        let left = -3.0 * Self::MAX_RADIUS;
        let right = screen_width() + 3.0 * Self::MAX_RADIUS;
        let bottom = screen_height() + 3.0 * Self::MAX_RADIUS;
        x < left || x > right || y > bottom
    }

    /// Обновление состояния астероида.
    pub fn update(&mut self, elapsed_time: f64, ship_speed: f32) {
        let elapsed_time = elapsed_time as f32;
        self.position += self.speed * elapsed_time;
        // Так как всё движется по вертикали в системе отсчёта корабля,
        // учтём его скорость.
        self.position.y += ship_speed * elapsed_time;
    }

    /// Отображение астероида.
    pub fn draw(&self) {
        // Отображаем астероид в виде красного круга.
        draw_circle(self.position.x, self.position.y, self.radius, LIGHTGRAY);
    }
}
