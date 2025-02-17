#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::eq_op)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::unnecessary_cast)]

use anchor_lang::prelude::*;

declare_id!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq");

#[program]
pub mod maze {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;

        state.x = 0;
        state.y = 0;

        Ok(())
    }
    pub fn move_north(
        ctx: Context<MoveNorth>,
        p0: u64,
        p1: u64,
        p2: u64,
        p3: u64,
        p4: u64,
        p5: u64,
        p6: u64,
        p7: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        let ny = state.y + 1;

        require!(ny < 7, MazeError::OutOfBounds);

        state.y = ny;

        match step(state.x, state.y, p0, p1, p2, p3, p4, p5, p6, p7) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub fn move_south(
        ctx: Context<MoveSouth>,
        p0: u64,
        p1: u64,
        p2: u64,
        p3: u64,
        p4: u64,
        p5: u64,
        p6: u64,
        p7: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        require!(state.y > 0, MazeError::OutOfBounds);

        let ny = state.y - 1;
        state.y = ny;

        match step(state.x, state.y, p0, p1, p2, p3, p4, p5, p6, p7) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub fn move_east(
        ctx: Context<MoveEast>,
        p0: u64,
        p1: u64,
        p2: u64,
        p3: u64,
        p4: u64,
        p5: u64,
        p6: u64,
        p7: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        let nx = state.x + 1;

        require!(nx < 7, MazeError::OutOfBounds);

        state.x = nx;

        match step(state.x, state.y, p0, p1, p2, p3, p4, p5, p6, p7) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub fn move_west(
        ctx: Context<MoveWest>,
        p0: u64,
        p1: u64,
        p2: u64,
        p3: u64,
        p4: u64,
        p5: u64,
        p6: u64,
        p7: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        require!(state.x > 0, MazeError::OutOfBounds);

        let nx = state.x - 1;
        state.x = nx;

        match step(state.x, state.y, p0, p1, p2, p3, p4, p5, p6, p7) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

fn step(
    x: u64,
    y: u64,
    p0: u64,
    p1: u64,
    p2: u64,
    p3: u64,
    p4: u64,
    p5: u64,
    p6: u64,
    p7: u64,
) -> Result<u64> {
    if x == 0 && y == 0 {
        // start
        return Ok(0);
    }
    if x == 0 && y == 1 {
        if p2 <= 55 {
            if p6 < (47 as u64).wrapping_mul(p7) {
                if p6 <= p1.wrapping_add(p0) {
                    if p3 > p0 {
                        if p5 == (39 as u64).wrapping_add(p5) {
                            if p6 < p7.wrapping_add(p3) {
                                panic!("AssertionFailed: 1"); //bug
                            }
                        }
                    }
                }
            }
        }
        return Ok(1);
    }
    if x == 0 && y == 2 {
        if p3 > (28 as u64).wrapping_mul(p7) {
            if p0 == p5.wrapping_mul(p1) {
                if p5 != (38 as u64).wrapping_add(p2) {
                    if p5 != (32 as u64).wrapping_add(p4) {
                        if p1 > (4 as u64).wrapping_mul(p4) {
                            panic!("AssertionFailed: 2"); //bug
                        }
                    }
                }
            }
        }
        return Ok(2);
    }
    if x == 0 && y == 3 {
        if p7 == p3.wrapping_add(p7) {
            if p5 != 8 {
                if p4 <= (26 as u64).wrapping_add(p2) {
                    if p2 != (59 as u64).wrapping_mul(p7) {
                        if p1 > 48 {
                            if p7 != (39 as u64).wrapping_add(p0) {
                                if p3 >= 8 {
                                    if p0 == (8 as u64).wrapping_add(p7) {
                                        if p0 >= p2.wrapping_mul(p0) {
                                            panic!("AssertionFailed: 3"); //bug
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(3);
    }
    if x == 0 && y == 4 {
        if p1 == 3 {
            if p0 <= p7 {
                if p0 > (35 as u64).wrapping_mul(p4) {
                    if p5 < p0.wrapping_add(p5) {
                        if p2 < p0.wrapping_add(p5) {
                            panic!("AssertionFailed: 4"); //bug
                        }
                    }
                }
            }
        }
        return Ok(4);
    }
    if x == 0 && y == 5 {
        if p2 < (64 as u64).wrapping_mul(p1) {
            if p7 < p4 {
                if p0 <= 57 {
                    if p2 != 48 {
                        if p3 != (32 as u64).wrapping_add(p3) {
                            if p4 <= p3.wrapping_add(p7) {
                                if p7 <= p5.wrapping_mul(p4) {
                                    if p0 <= p7.wrapping_mul(p0) {
                                        if p4 != p1 {
                                            if p0 >= p0.wrapping_add(p3) {
                                                if p5 < (57 as u64).wrapping_mul(p5) {
                                                    if p4 < p2 {
                                                        if p6 > p1.wrapping_mul(p5) {
                                                            panic!("AssertionFailed: 5");
                                                            //bug
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(5);
    }
    if x == 0 && y == 6 {
        if p3 == p5 {
            if p0 <= 0 {
                if p7 > (1 as u64).wrapping_add(p3) {
                    if p1 == p4.wrapping_mul(p5) {
                        if p0 == p6.wrapping_mul(p1) {
                            if p6 != p1.wrapping_mul(p3) {
                                if p4 > (28 as u64).wrapping_add(p7) {
                                    if p5 <= p0.wrapping_add(p5) {
                                        panic!("AssertionFailed: 6"); //bug
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(6);
    }
    if x == 1 && y == 0 {
        msg!("Wall: {}", 7);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 1 && y == 1 {
        if p1 == (58 as u64).wrapping_add(p7) {
            if p2 < p6.wrapping_mul(p5) {
                if p2 == (56 as u64).wrapping_mul(p2) {
                    if p5 != (33 as u64).wrapping_add(p1) {
                        if p6 > (17 as u64).wrapping_add(p5) {
                            if p7 <= p2 {
                                if p0 < 33 {
                                    if p5 > p1.wrapping_mul(p1) {
                                        if p6 <= (48 as u64).wrapping_add(p0) {
                                            if p0 >= (1 as u64).wrapping_mul(p7) {
                                                if p4 == (2 as u64).wrapping_add(p3) {
                                                    if p1 >= 52 {
                                                        panic!("AssertionFailed: 8");
                                                        //bug
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(8);
    }
    if x == 1 && y == 2 {
        if p2 < p4 {
            if p1 > (28 as u64).wrapping_mul(p6) {
                if p2 == 57 {
                    if p5 >= p3.wrapping_add(p5) {
                        if p7 != (49 as u64).wrapping_mul(p3) {
                            if p4 < p2.wrapping_add(p0) {
                                if p5 >= (15 as u64).wrapping_mul(p0) {
                                    if p3 > p0.wrapping_add(p2) {
                                        if p0 < (44 as u64).wrapping_mul(p2) {
                                            if p2 > (49 as u64).wrapping_mul(p7) {
                                                panic!("AssertionFailed: 9"); //bug
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(9);
    }
    if x == 1 && y == 3 {
        if p6 == (6 as u64).wrapping_mul(p0) {
            if p0 >= p4 {
                if p0 <= p2.wrapping_add(p5) {
                    if p4 <= p1.wrapping_add(p7) {
                        if p4 >= p4.wrapping_add(p3) {
                            if p7 != p2 {
                                if p4 > p1 {
                                    if p0 <= 46 {
                                        if p6 < p0.wrapping_add(p3) {
                                            if p5 < p6.wrapping_mul(p4) {
                                                if p0 >= (19 as u64).wrapping_add(p3) {
                                                    if p6 <= 29 {
                                                        if p4 >= p6.wrapping_mul(p5) {
                                                            panic!("AssertionFailed: 10");
                                                            //bug
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(10);
    }
    if x == 1 && y == 4 {
        if p0 < p4.wrapping_mul(p3) {
            if p2 > p2.wrapping_add(p2) {
                if p4 >= (36 as u64).wrapping_add(p1) {
                    if p4 >= (28 as u64).wrapping_add(p4) {
                        if p3 > p1.wrapping_add(p0) {
                            if p3 != p7.wrapping_mul(p4) {
                                if p3 == 40 {
                                    if p1 > (21 as u64).wrapping_add(p7) {
                                        if p6 != (6 as u64).wrapping_mul(p6) {
                                            if p4 != p2.wrapping_mul(p0) {
                                                if p1 != p6 {
                                                    if p1 < p6.wrapping_mul(p6) {
                                                        if p5 == p2.wrapping_mul(p6) {
                                                            if p1 >= (54 as u64).wrapping_add(p0) {
                                                                panic!("AssertionFailed: 11");
                                                                //bug
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(11);
    }
    if x == 1 && y == 5 {
        if p6 < 30 {
            if p6 >= p2.wrapping_add(p4) {
                if p0 != (4 as u64).wrapping_add(p7) {
                    if p1 < p0.wrapping_add(p7) {
                        if p4 > (26 as u64).wrapping_add(p7) {
                            if p1 <= p1 {
                                if p6 != p6 {
                                    if p5 >= p2.wrapping_add(p2) {
                                        if p4 != (12 as u64).wrapping_mul(p3) {
                                            if p6 <= (4 as u64).wrapping_add(p1) {
                                                if p6 <= 38 {
                                                    panic!("AssertionFailed: 12");
                                                    //bug
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(12);
    }
    if x == 1 && y == 6 {
        if p7 < p5 {
            if p2 > p5.wrapping_add(p2) {
                if p4 == 2 {
                    if p5 <= 35 {
                        if p1 > p0 {
                            if p6 > (52 as u64).wrapping_add(p2) {
                                if p7 <= (55 as u64).wrapping_add(p6) {
                                    panic!("AssertionFailed: 13"); //bug
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(13);
    }
    if x == 2 && y == 0 {
        if p7 >= p4.wrapping_add(p3) {
            if p2 != (49 as u64).wrapping_add(p5) {
                if p2 > p1 {
                    if p3 > p3.wrapping_add(p0) {
                        if p1 != p0.wrapping_mul(p1) {
                            if p0 >= p3 {
                                if p7 > p4 {
                                    if p0 > 31 {
                                        if p1 >= p6.wrapping_mul(p5) {
                                            panic!("AssertionFailed: 14"); //bug
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(14);
    }
    if x == 2 && y == 1 {
        if p4 <= p5 {
            if p5 > p1.wrapping_add(p1) {
                if p2 <= (55 as u64).wrapping_mul(p1) {
                    if p3 == 14 {
                        if p4 <= p3 {
                            if p4 != (46 as u64).wrapping_mul(p6) {
                                if p2 != 40 {
                                    if p7 != 58 {
                                        if p7 == p1 {
                                            if p1 == (1 as u64).wrapping_mul(p4) {
                                                if p1 < (2 as u64).wrapping_add(p4) {
                                                    if p3 > p0 {
                                                        if p5 != (63 as u64).wrapping_add(p0) {
                                                            panic!("AssertionFailed: 15");
                                                            //bug
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(15);
    }
    if x == 2 && y == 2 {
        if p7 <= p1 {
            if p3 <= p6 {
                if p5 < p4 {
                    if p2 > (4 as u64).wrapping_mul(p1) {
                        if p1 == p3.wrapping_add(p4) {
                            if p1 >= p4.wrapping_mul(p2) {
                                if p1 != (28 as u64).wrapping_mul(p7) {
                                    if p0 == (48 as u64).wrapping_add(p0) {
                                        if p7 != (36 as u64).wrapping_add(p3) {
                                            if p7 <= 17 {
                                                if p2 <= p6.wrapping_mul(p3) {
                                                    if p3 < p5.wrapping_add(p4) {
                                                        if p2 >= p5 {
                                                            if p4 == (19 as u64).wrapping_mul(p0) {
                                                                if p0
                                                                    == (44 as u64).wrapping_add(p1)
                                                                {
                                                                    if p6 == p3 {
                                                                        panic!(
                                                                            "AssertionFailed: 16"
                                                                        ); //bug
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(16);
    }
    if x == 2 && y == 3 {
        if p6 >= (1 as u64).wrapping_add(p1) {
            if p7 > 7 {
                if p7 > p4.wrapping_add(p3) {
                    if p5 > (52 as u64).wrapping_add(p4) {
                        if p1 >= (52 as u64).wrapping_add(p2) {
                            if p5 > (37 as u64).wrapping_add(p5) {
                                if p3 >= 33 {
                                    if p3 == p3.wrapping_mul(p6) {
                                        if p4 != p4.wrapping_mul(p6) {
                                            if p5 >= (6 as u64).wrapping_add(p1) {
                                                if p3 <= 25 {
                                                    panic!("AssertionFailed: 17");
                                                    //bug
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(17);
    }
    if x == 2 && y == 4 {
        if p6 < 6 {
            if p5 > (54 as u64).wrapping_mul(p3) {
                if p5 >= p7.wrapping_add(p1) {
                    if p3 <= p7 {
                        if p5 < p2.wrapping_add(p5) {
                            if p0 != (10 as u64).wrapping_mul(p7) {
                                if p4 == p4 {
                                    if p1 == (37 as u64).wrapping_add(p2) {
                                        if p4 == 30 {
                                            panic!("AssertionFailed: 18"); //bug
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(18);
    }
    if x == 2 && y == 5 {
        if p5 <= (55 as u64).wrapping_add(p3) {
            if p2 <= p3.wrapping_mul(p5) {
                if p5 >= p3.wrapping_mul(p6) {
                    if p3 > (37 as u64).wrapping_mul(p2) {
                        if p6 > p7.wrapping_mul(p6) {
                            if p1 < (56 as u64).wrapping_add(p6) {
                                if p4 <= (21 as u64).wrapping_mul(p2) {
                                    if p2 != p5.wrapping_add(p6) {
                                        if p5 != (46 as u64).wrapping_mul(p4) {
                                            if p4 >= (19 as u64).wrapping_mul(p3) {
                                                panic!("AssertionFailed: 19"); //bug
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(19);
    }
    if x == 2 && y == 6 {
        if p3 >= p1.wrapping_add(p4) {
            if p2 <= p3.wrapping_mul(p4) {
                if p0 <= (9 as u64).wrapping_mul(p7) {
                    if p0 == 59 {
                        if p6 != p2.wrapping_mul(p0) {
                            if p3 > p3 {
                                if p0 != (53 as u64).wrapping_add(p0) {
                                    if p2 != (45 as u64).wrapping_mul(p2) {
                                        if p2 <= (27 as u64).wrapping_add(p2) {
                                            if p5 > p6.wrapping_mul(p5) {
                                                if p0 > p7.wrapping_add(p7) {
                                                    if p3 > 6 {
                                                        if p6 >= (49 as u64).wrapping_add(p5) {
                                                            panic!("AssertionFailed: 20");
                                                            //bug
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(20);
    }
    if x == 3 && y == 0 {
        if p1 > p5.wrapping_mul(p2) {
            if p2 == p4.wrapping_mul(p1) {
                if p2 < p5 {
                    if p4 >= p0.wrapping_add(p7) {
                        if p2 > 14 {
                            if p3 <= p3.wrapping_add(p4) {
                                if p2 != p1.wrapping_mul(p6) {
                                    if p6 <= (43 as u64).wrapping_add(p0) {
                                        if p4 <= (19 as u64).wrapping_add(p7) {
                                            if p1 != p1 {
                                                if p1 >= p2 {
                                                    if p3 <= p4.wrapping_mul(p1) {
                                                        if p6 <= p3.wrapping_add(p5) {
                                                            if p0 < p2.wrapping_mul(p3) {
                                                                if p5 >= p3.wrapping_add(p7) {
                                                                    if p4 != p4.wrapping_mul(p0) {
                                                                        panic!(
                                                                            "AssertionFailed: 21"
                                                                        ); //bug
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(21);
    }
    if x == 3 && y == 1 {
        if p3 != (13 as u64).wrapping_mul(p3) {
            if p3 < 40 {
                if p6 == p4.wrapping_add(p1) {
                    if p6 < (39 as u64).wrapping_add(p0) {
                        if p1 != (29 as u64).wrapping_add(p6) {
                            if p0 < p6 {
                                if p5 > p2.wrapping_add(p4) {
                                    if p4 < p3 {
                                        if p5 == (24 as u64).wrapping_mul(p0) {
                                            if p4 != p6.wrapping_add(p0) {
                                                panic!("AssertionFailed: 22"); //bug
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(22);
    }
    if x == 3 && y == 2 {
        if p6 < (40 as u64).wrapping_mul(p6) {
            if p2 == p1.wrapping_add(p6) {
                if p6 != p4 {
                    if p4 >= p6.wrapping_add(p3) {
                        if p2 >= p0 {
                            if p6 < 58 {
                                if p3 <= p4 {
                                    if p1 <= 50 {
                                        if p7 > p5.wrapping_add(p4) {
                                            if p3 < p3.wrapping_mul(p4) {
                                                if p2 == 20 {
                                                    if p7 > 3 {
                                                        if p4 == p1 {
                                                            if p6 > p7.wrapping_mul(p1) {
                                                                panic!("AssertionFailed: 23");
                                                                //bug
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(23);
    }
    if x == 3 && y == 3 {
        if p6 >= (35 as u64).wrapping_mul(p2) {
            if p2 != p4.wrapping_add(p3) {
                if p1 < 56 {
                    if p2 <= (20 as u64).wrapping_add(p4) {
                        if p4 < (47 as u64).wrapping_mul(p1) {
                            if p5 == (49 as u64).wrapping_mul(p0) {
                                if p3 != (43 as u64).wrapping_mul(p4) {
                                    if p1 == p4.wrapping_mul(p3) {
                                        if p0 > 20 {
                                            if p2 < (54 as u64).wrapping_add(p5) {
                                                if p7 == p4.wrapping_add(p6) {
                                                    if p2 <= (16 as u64).wrapping_mul(p3) {
                                                        panic!("AssertionFailed: 24");
                                                        //bug
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(24);
    }
    if x == 3 && y == 4 {
        if p3 >= 10 {
            if p0 <= p6 {
                if p2 < p0.wrapping_mul(p2) {
                    if p6 > p3.wrapping_add(p0) {
                        if p0 > p4.wrapping_add(p6) {
                            if p6 >= p0.wrapping_add(p4) {
                                if p3 != p7.wrapping_add(p5) {
                                    if p4 > (47 as u64).wrapping_mul(p0) {
                                        if p4 == p6 {
                                            if p6 < p6.wrapping_mul(p4) {
                                                if p4 != (49 as u64).wrapping_add(p3) {
                                                    if p4 >= p3 {
                                                        panic!("AssertionFailed: 25");
                                                        //bug
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(25);
    }
    if x == 3 && y == 5 {
        if p4 > 22 {
            if p7 > p2.wrapping_mul(p1) {
                panic!("AssertionFailed: 26"); //bug
            }
        }
        return Ok(26);
    }
    if x == 3 && y == 6 {
        if p1 <= (4 as u64).wrapping_add(p3) {
            if p6 > 62 {
                if p4 >= (4 as u64).wrapping_add(p1) {
                    panic!("AssertionFailed: 27"); //bug
                }
            }
        }
        return Ok(27);
    }
    if x == 4 && y == 0 {
        if p4 > p0.wrapping_add(p0) {
            if p7 != (61 as u64).wrapping_mul(p5) {
                if p0 >= p0.wrapping_mul(p3) {
                    if p6 == 50 {
                        if p0 >= p4.wrapping_add(p3) {
                            if p3 < (37 as u64).wrapping_add(p5) {
                                if p6 == (14 as u64).wrapping_mul(p2) {
                                    if p5 > (16 as u64).wrapping_mul(p3) {
                                        if p6 != p4 {
                                            if p1 == 35 {
                                                panic!("AssertionFailed: 28"); //bug
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(28);
    }
    if x == 4 && y == 1 {
        if p5 == (57 as u64).wrapping_add(p0) {
            if p6 >= p7 {
                if p1 < p1.wrapping_add(p6) {
                    if p1 <= p6.wrapping_mul(p4) {
                        panic!("AssertionFailed: 29"); //bug
                    }
                }
            }
        }
        return Ok(29);
    }
    if x == 4 && y == 2 {
        if p7 == (35 as u64).wrapping_add(p2) {
            if p4 != p2.wrapping_mul(p1) {
                if p4 != p1 {
                    if p4 > p3.wrapping_add(p2) {
                        if p2 < p6.wrapping_add(p0) {
                            if p3 < p1.wrapping_add(p6) {
                                if p1 == p5.wrapping_add(p2) {
                                    if p0 == p3.wrapping_mul(p2) {
                                        if p1 < p7 {
                                            if p0 <= 37 {
                                                if p2 < (37 as u64).wrapping_add(p3) {
                                                    panic!("AssertionFailed: 30");
                                                    //bug
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(30);
    }
    if x == 4 && y == 3 {
        if p4 <= p0.wrapping_mul(p5) {
            if p3 > p6.wrapping_add(p4) {
                if p7 < p3.wrapping_add(p2) {
                    if p3 < (47 as u64).wrapping_mul(p3) {
                        if p1 > p7.wrapping_add(p7) {
                            if p3 > p7.wrapping_mul(p4) {
                                panic!("AssertionFailed: 31"); //bug
                            }
                        }
                    }
                }
            }
        }
        return Ok(31);
    }
    if x == 4 && y == 4 {
        msg!("Wall: {}", 32);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 4 && y == 5 {
        if p3 != (28 as u64).wrapping_mul(p0) {
            if p6 != 10 {
                if p7 != 3 {
                    if p6 > p1.wrapping_mul(p6) {
                        panic!("AssertionFailed: 33"); //bug
                    }
                }
            }
        }
        return Ok(33);
    }
    if x == 4 && y == 6 {
        if p0 < (30 as u64).wrapping_add(p7) {
            if p0 >= p6 {
                if p1 != (40 as u64).wrapping_mul(p3) {
                    if p6 != 29 {
                        if p5 > p4.wrapping_mul(p3) {
                            if p1 != (52 as u64).wrapping_add(p0) {
                                if p0 != p1 {
                                    if p4 == (23 as u64).wrapping_add(p4) {
                                        if p5 <= (19 as u64).wrapping_add(p1) {
                                            if p1 == p0.wrapping_add(p5) {
                                                if p3 <= p6.wrapping_mul(p0) {
                                                    if p4 != 62 {
                                                        panic!("AssertionFailed: 34");
                                                        //bug
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(34);
    }
    if x == 5 && y == 0 {
        if p4 > p1.wrapping_mul(p6) {
            if p1 > p7.wrapping_mul(p2) {
                if p4 != 33 {
                    if p5 == (0 as u64).wrapping_add(p6) {
                        if p6 > p5.wrapping_add(p5) {
                            if p7 > p5.wrapping_mul(p3) {
                                if p6 <= 12 {
                                    panic!("AssertionFailed: 35"); //bug
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(35);
    }
    if x == 5 && y == 1 {
        if p7 > p2 {
            if p0 < 36 {
                if p6 != (15 as u64).wrapping_mul(p6) {
                    if p4 != (15 as u64).wrapping_add(p3) {
                        if p7 <= p2 {
                            if p5 > p2 {
                                if p6 == p1.wrapping_add(p5) {
                                    if p5 > p5.wrapping_add(p5) {
                                        panic!("AssertionFailed: 36"); //bug
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(36);
    }
    if x == 5 && y == 2 {
        if p6 >= p0.wrapping_add(p6) {
            if p5 > 5 {
                if p5 != (1 as u64).wrapping_mul(p5) {
                    if p5 < 53 {
                        if p1 >= (3 as u64).wrapping_add(p6) {
                            if p1 == (44 as u64).wrapping_mul(p7) {
                                if p4 < (21 as u64).wrapping_mul(p2) {
                                    if p2 >= (60 as u64).wrapping_mul(p6) {
                                        if p2 < 40 {
                                            if p0 >= p7.wrapping_mul(p4) {
                                                if p1 != p5.wrapping_mul(p1) {
                                                    if p3 < (19 as u64).wrapping_add(p0) {
                                                        panic!("AssertionFailed: 37");
                                                        //bug
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(37);
    }
    if x == 5 && y == 3 {
        if p5 >= p6.wrapping_mul(p4) {
            if p7 <= 54 {
                panic!("AssertionFailed: 38"); //bug
            }
        }
        return Ok(38);
    }
    if x == 5 && y == 4 {
        if p1 >= p1.wrapping_mul(p6) {
            if p4 == p0.wrapping_mul(p7) {
                panic!("AssertionFailed: 39"); //bug
            }
        }
        return Ok(39);
    }
    if x == 5 && y == 5 {
        if p1 > (26 as u64).wrapping_add(p0) {
            if p5 < (23 as u64).wrapping_mul(p5) {
                panic!("AssertionFailed: 40"); //bug
            }
        }
        return Ok(40);
    }
    if x == 5 && y == 6 {
        if p0 < 39 {
            if p2 != 13 {
                if p2 == (61 as u64).wrapping_add(p0) {
                    if p1 == p6 {
                        if p6 == (24 as u64).wrapping_add(p3) {
                            if p0 >= (42 as u64).wrapping_mul(p7) {
                                if p3 < (37 as u64).wrapping_add(p2) {
                                    panic!("AssertionFailed: 41"); //bug
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(41);
    }
    if x == 6 && y == 0 {
        if p0 != p6.wrapping_add(p2) {
            if p5 <= p5.wrapping_mul(p7) {
                if p7 <= (15 as u64).wrapping_add(p6) {
                    panic!("AssertionFailed: 42"); //bug
                }
            }
        }
        return Ok(42);
    }
    if x == 6 && y == 1 {
        if p1 <= 52 {
            if p4 <= (35 as u64).wrapping_add(p1) {
                panic!("AssertionFailed: 43"); //bug
            }
        }
        return Ok(43);
    }
    if x == 6 && y == 2 {
        if p6 >= p7 {
            if p4 <= p5.wrapping_add(p5) {
                if p7 >= (3 as u64).wrapping_add(p0) {
                    if p6 < (60 as u64).wrapping_mul(p2) {
                        if p1 < (39 as u64).wrapping_mul(p3) {
                            if p5 != p2 {
                                panic!("AssertionFailed: 44"); //bug
                            }
                        }
                    }
                }
            }
        }
        return Ok(44);
    }
    if x == 6 && y == 3 {
        if p5 >= (34 as u64).wrapping_mul(p6) {
            if p0 != 63 {
                if p5 < (40 as u64).wrapping_mul(p3) {
                    if p5 == p5 {
                        if p5 > p6 {
                            if p3 >= 31 {
                                if p7 < p3.wrapping_add(p7) {
                                    if p1 > p7.wrapping_add(p1) {
                                        if p6 == (63 as u64).wrapping_add(p6) {
                                            if p0 >= p1.wrapping_add(p2) {
                                                if p0 == p6.wrapping_mul(p7) {
                                                    if p4 < 55 {
                                                        if p3 != 7 {
                                                            if p1 < p6.wrapping_add(p3) {
                                                                if p0 < (7 as u64).wrapping_mul(p4)
                                                                {
                                                                    if p6
                                                                        <= (52 as u64)
                                                                            .wrapping_add(p3)
                                                                    {
                                                                        panic!(
                                                                            "AssertionFailed: 45"
                                                                        ); //bug
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(45);
    }
    if x == 6 && y == 4 {
        if p7 != p0.wrapping_add(p7) {
            if p2 > p2.wrapping_mul(p1) {
                if p7 != (30 as u64).wrapping_mul(p1) {
                    if p6 >= p2.wrapping_mul(p0) {
                        panic!("AssertionFailed: 46"); //bug
                    }
                }
            }
        }
        return Ok(46);
    }
    if x == 6 && y == 5 {
        if p3 == p5.wrapping_add(p1) {
            if p0 != p0 {
                if p3 == (37 as u64).wrapping_mul(p0) {
                    if p6 >= p6 {
                        if p2 < p1.wrapping_mul(p4) {
                            if p4 != p3.wrapping_add(p5) {
                                if p1 <= p4.wrapping_add(p7) {
                                    if p1 >= (10 as u64).wrapping_mul(p3) {
                                        if p4 != p7.wrapping_add(p6) {
                                            panic!("AssertionFailed: 47"); //bug
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(47);
    }
    if x == 6 && y == 6 {
        msg!("Wall: {}", 48);
        return Err(MazeError::Wall.into()); // wall
    }
    return Ok(49);
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub state_author: Signer<'info>,
    #[account(init, payer = state_author, space = 8 + State::LEN, seeds = ["state".as_bytes()], bump)]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveNorth<'info> {
    #[account(
        mut,
        seeds = ["state".as_bytes()],
        bump
    )]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct MoveSouth<'info> {
    #[account(
        mut,
        seeds = ["state".as_bytes()],
        bump
    )]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct MoveEast<'info> {
    #[account(
        mut,
        seeds = ["state".as_bytes()],
        bump
    )]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct MoveWest<'info> {
    #[account(
        mut,
        seeds = ["state".as_bytes()],
        bump
    )]
    pub state: Account<'info, State>,
}

#[account]
pub struct State {
    pub x: u64,
    pub y: u64,
}
impl State {
    pub const LEN: usize = 8 + 8;
}

#[error_code]
pub enum MazeError {
    #[msg("Attempt to move outside of the boundaries.")]
    OutOfBounds,
    #[msg("Encountered a wall.")]
    Wall,
}
