use bracket_lib::prelude::*;

// 游戏菜单
enum GameMode {
    Menu,
    Playing,
    End,
}

// 常量
// 屏幕宽度
const SCREEN_WIDTH: i32 = 80;
// 屏幕高度
const SCREEN_HEIGHT: i32 = 50;
// todo 刷新时间？
const FRAME_DURATION: f32 = 25.0;

// 游戏状态
struct State {
    mode: GameMode,
    player: Player,
    // 经过多少帧之后，累计了多少时间
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

// 玩家
struct Player {
    x: i32,
    y: i32,
    // 重力方向的速度，小于0就会下落
    velocity: f32,
}

impl Player {
    // 关联函数
    fn new(x: i32, y: i32) -> Self {
        Player {
            x: 0,
            y: 0,
            velocity: 0.0,
        }
    }

    // 玩家操控的实体
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    // 重力公式
    fn gravity_to_move(&mut self) {
        if self.velocity < 1.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        // 超出屏幕最上方，重定位
        if self.y < 0 {
            self.y = 0
        }
    }

    // 起飞操作
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

impl State {
    // 关联函数
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    // 主菜单
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    // 重新开始游戏
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    // 开始游戏
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_to_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);

        ctx.print(0, 0, "press space to flap");
        ctx.print(0, 1, &format!("Score:{}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score)
        }

        // 落地或者碰到障碍物
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    // 游戏结束
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "you are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

// 游戏状态
impl GameState for State {
    // 根据用户输入选择菜单
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

// 障碍物
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    // 关联函数
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    // 随机生成障碍物
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    // 碰到障碍物
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

fn main() -> BError {
    // 实例化游戏页面
    let context = BTermBuilder::simple80x50()
        .with_title("flappy dragon")
        .build()?;

    // 开始轮询
    main_loop(context, State::new())
}
