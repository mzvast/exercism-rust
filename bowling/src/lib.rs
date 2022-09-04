#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// 一共有10轮，每轮可以扔1-2次
// strike 全中，得分为 10 + 接下来2次的得分
// spare 补中，得分为10 + 接下来1次投掷到得分
// open frame 第二次还有剩，得分为本轮一共击倒的个数

// 第10轮的如果扔出全中或者补中，会额外增加一次机会(本轮一共3次)，该轮的得分为本轮总击倒的个数
#[derive(Debug)]
enum FrameResult {
    None,
    Strike,
    Spare,
    OpenFrame,
}

impl Default for FrameResult {
    fn default() -> Self {
        FrameResult::None
    }
}

#[derive(Default, Debug)]
struct Frame {
    index: i8,      // 第i轮
    data: Vec<u16>, // 具体击倒数据
    // computed
    pins_left: u16,      // 还剩几个pins [0,10]
    result: FrameResult, // 该轮比赛结果
    is_end: bool,        // 本轮是否结束
    is_fill: bool,       // 是否fillBalls
}

impl Frame {
    fn new(index: i8) -> Self {
        Frame {
            index,
            pins_left: 10,
            ..Default::default()
        }
    }
    pub fn hit(&mut self, pins: u16) -> Result<(), Error> {
        if self.pins_left < pins {
            Err(Error::NotEnoughPinsLeft)
        } else {
            self.pins_left = self.pins_left - pins;
            self.data.push(pins);
            self.update_result();
            Ok(())
        }
    }

    fn update_result(&mut self) {
        if self.index == 9 {
            // 10th frame
            // trigger fill ball by strike or spare
            if self.pins_left == 0 && self.data.len() <= 2 {
                // fill ball
                self.is_fill = true;
                self.pins_left = 10;
                return;
            }

            // fill ball end
            if self.is_fill {
                if self.data.len() == 3 {
                    self.is_end = true;
                    return;
                }
            } else {
                // normal ending
                if self.data.len() == 2 {
                    self.is_end = true;
                    return;
                }
            }
        } else {
            // normal frame
            // clean
            if self.pins_left == 0 {
                self.is_end = true;
                self.result = match self.data.len() {
                    1 => FrameResult::Strike,
                    2 => FrameResult::Spare,
                    _ => panic!(),
                };
                return;
            }
            // not clean
            if self.data.len() == 2 {
                self.is_end = true;
                self.result = FrameResult::OpenFrame;
            }
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>, // 所有frame
    cur_idx: i8,        // 当前的frame [0,9]
    is_complete: bool,  // 游戏已经结束
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::new(),
            cur_idx: -1,
            is_complete: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        println!("Record that {} pins have been scored", pins);
        if self.is_complete {
            return Err(Error::GameComplete);
        }
        let mut f = self.pop_frame(); // 当前的frame
        match f.hit(pins) {
            Ok(_) => {
                self.push_frame(f);
                Ok(())
            }
            Err(reason) => Err(reason),
        }
    }

    fn pop_frame(&mut self) -> Frame {
        if self.frames.is_empty() || self.frames.last().unwrap().is_end {
            // create new frame if last frame is empty or is end
            self.cur_idx += 1;
            Frame::new(self.cur_idx)
        } else {
            // get the last frame and delete it
            self.frames.pop().unwrap()
        }
    }

    fn push_frame(&mut self, f: Frame) {
        self.frames.push(f);
        if self.cur_idx == 9 && self.frames.last().unwrap().is_end {
            self.is_complete = true;
        }
    }

    pub fn score(&self) -> Option<u16> {
        println!("{:#?}", self);
        if !self.is_complete {
            return None;
        }

        let mut ans = 0;

        for i in 0..10 {
            // [0,9]
            let cur = &self.frames[i];
            match cur.result {
                FrameResult::Strike => {
                    // add next 2
                    ans += 10;
                    if self.frames[i + 1].data.len() > 1 {
                        ans += self.frames[i + 1].data[0];
                        ans += self.frames[i + 1].data[1];
                    } else {
                        ans += self.frames[i + 1].data[0];
                        ans += self.frames[i + 2].data[0];
                    }
                }
                FrameResult::Spare => {
                    ans += 10;
                    // add next 1
                    ans += self.frames[i + 1].data[0];
                }
                FrameResult::OpenFrame => {
                    ans += cur.data.iter().sum::<u16>();
                }
                FrameResult::None => {
                    // 10th frame
                    ans += cur.data.iter().sum::<u16>();
                }
            }
        }

        Some(ans)
    }
}
