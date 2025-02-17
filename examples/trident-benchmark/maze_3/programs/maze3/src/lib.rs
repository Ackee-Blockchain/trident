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
        if p1 != 26 {
            if p3 < 27 {
                if p3 > (8 as u64).wrapping_add(p5) {
                    if p6 <= p2.wrapping_mul(p6) {
                        if p6 == p7.wrapping_add(p1) {
                            panic!("AssertionFailed: 1"); // bug
                        }
                    }
                }
            }
        }
        return Ok(1);
    }
    if x == 0 && y == 2 {
        if p5 == p6.wrapping_add(p0) {
            if p2 <= p6 {
                if p4 <= (33 as u64).wrapping_mul(p5) {
                    if p1 <= p0.wrapping_mul(p5) {
                        if p5 != 21 {
                            if p1 > 13 {
                                if p1 < (50 as u64).wrapping_mul(p1) {
                                    if p6 >= (6 as u64).wrapping_mul(p0) {
                                        if p5 != (64 as u64).wrapping_mul(p7) {
                                            if p3 > p3.wrapping_mul(p7) {
                                                panic!("AssertionFailed: 2"); // bug
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
        return Ok(2);
    }
    if x == 0 && y == 3 {
        if p2 <= p4.wrapping_add(p5) {
            if p0 <= (38 as u64).wrapping_mul(p7) {
                panic!("AssertionFailed: 3"); // bug
            }
        }
        return Ok(3);
    }
    if x == 0 && y == 4 {
        msg!("Wall: {}", 4);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 0 && y == 5 {
        msg!("Wall: {}", 5);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 0 && y == 6 {
        if p0 == 48 {
            if p5 > p4.wrapping_mul(p7) {
                panic!("AssertionFailed: 6"); // bug
            }
        }
        return Ok(6);
    }
    if x == 1 && y == 0 {
        if p1 < p4.wrapping_add(p3) {
            if p1 > (27 as u64).wrapping_add(p1) {
                if p6 < 52 {
                    if p4 >= (56 as u64).wrapping_add(p7) {
                        if p5 > 6 {
                            if p1 < p7.wrapping_mul(p0) {
                                if p5 >= (58 as u64).wrapping_add(p7) {
                                    panic!("AssertionFailed: 7"); // bug
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(7);
    }
    if x == 1 && y == 1 {
        if p4 <= p7 {
            if p5 != (13 as u64).wrapping_mul(p7) {
                panic!("AssertionFailed: 8"); // bug
            }
        }
        return Ok(8);
    }
    if x == 1 && y == 2 {
        if p6 < (57 as u64).wrapping_add(p1) {
            if p6 > 47 {
                if p1 <= (58 as u64).wrapping_mul(p1) {
                    if p2 <= (33 as u64).wrapping_add(p3) {
                        if p3 >= p3 {
                            if p4 != (26 as u64).wrapping_add(p7) {
                                if p2 > (56 as u64).wrapping_mul(p2) {
                                    if p0 != p2.wrapping_mul(p3) {
                                        if p1 <= p0.wrapping_add(p1) {
                                            panic!("AssertionFailed: 9"); // bug
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
        if p3 >= p3.wrapping_add(p1) {
            if p7 != (40 as u64).wrapping_add(p6) {
                if p0 == p4 {
                    if p6 > (29 as u64).wrapping_add(p3) {
                        if p6 >= (55 as u64).wrapping_add(p3) {
                            if p0 < p5 {
                                if p2 <= p6.wrapping_mul(p6) {
                                    if p7 <= p4.wrapping_add(p6) {
                                        if p2 >= 24 {
                                            if p5 > p7.wrapping_mul(p3) {
                                                if p3 <= p6.wrapping_mul(p2) {
                                                    panic!("AssertionFailed: 10");
                                                    // bug
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
        if p5 != 59 {
            if p7 != p4 {
                if p4 != p5 {
                    if p1 == (42 as u64).wrapping_add(p4) {
                        if p0 >= p2 {
                            if p7 > (34 as u64).wrapping_mul(p6) {
                                if p2 > 2 {
                                    if p7 > p4.wrapping_add(p5) {
                                        panic!("AssertionFailed: 11"); // bug
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
        if p7 == 8 {
            if p3 >= p1 {
                if p4 > p0.wrapping_add(p6) {
                    panic!("AssertionFailed: 12"); // bug
                }
            }
        }
        return Ok(12);
    }
    if x == 1 && y == 6 {
        if p2 == p4 {
            if p3 > (22 as u64).wrapping_mul(p7) {
                if p4 > 61 {
                    if p1 == (59 as u64).wrapping_add(p5) {
                        if p1 <= p4.wrapping_add(p7) {
                            if p0 < p6.wrapping_mul(p4) {
                                if p4 >= (3 as u64).wrapping_mul(p0) {
                                    if p4 != (48 as u64).wrapping_add(p3) {
                                        if p0 != (52 as u64).wrapping_add(p6) {
                                            if p2 == p7 {
                                                if p5 >= p1 {
                                                    if p3 > (45 as u64).wrapping_mul(p5) {
                                                        if p5 <= p1.wrapping_mul(p0) {
                                                            panic!("AssertionFailed: 13");
                                                            // bug
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
        return Ok(13);
    }
    if x == 2 && y == 0 {
        if p7 == (8 as u64).wrapping_mul(p6) {
            if p5 >= p1 {
                if p6 > 50 {
                    if p7 != 51 {
                        if p2 >= p5.wrapping_mul(p2) {
                            if p2 < p6.wrapping_add(p7) {
                                if p7 > p7.wrapping_mul(p3) {
                                    if p5 == p3.wrapping_add(p3) {
                                        if p4 == (40 as u64).wrapping_add(p6) {
                                            if p1 < p4.wrapping_add(p6) {
                                                if p3 <= (21 as u64).wrapping_add(p4) {
                                                    if p1 != p3.wrapping_add(p1) {
                                                        if p4 > p4 {
                                                            if p1 == (28 as u64).wrapping_mul(p3) {
                                                                if p3 > p6 {
                                                                    panic!("AssertionFailed: 14");
                                                                    // bug
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
        return Ok(14);
    }
    if x == 2 && y == 1 {
        if p6 == p0.wrapping_add(p4) {
            if p6 <= p0 {
                if p1 < (46 as u64).wrapping_mul(p4) {
                    if p4 == 22 {
                        if p6 == (64 as u64).wrapping_mul(p3) {
                            if p6 == (10 as u64).wrapping_mul(p4) {
                                if p6 > (6 as u64).wrapping_mul(p4) {
                                    if p3 >= p3.wrapping_add(p5) {
                                        if p1 != (56 as u64).wrapping_add(p5) {
                                            if p2 <= p0.wrapping_add(p4) {
                                                if p5 < p5.wrapping_mul(p2) {
                                                    if p4 != p5.wrapping_mul(p3) {
                                                        if p1 >= 5 {
                                                            if p4 == 61 {
                                                                if p0 <= 42 {
                                                                    panic!("AssertionFailed: 15");
                                                                    // bug
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
        return Ok(15);
    }
    if x == 2 && y == 2 {
        if p6 <= p3.wrapping_mul(p4) {
            if p6 < (62 as u64).wrapping_add(p2) {
                if p4 >= p0 {
                    if p3 <= (56 as u64).wrapping_add(p7) {
                        if p0 >= (32 as u64).wrapping_add(p2) {
                            if p1 >= p0.wrapping_mul(p2) {
                                if p0 != (43 as u64).wrapping_mul(p5) {
                                    if p5 == (63 as u64).wrapping_mul(p5) {
                                        panic!("AssertionFailed: 16"); // bug
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
        if p6 != p7 {
            if p4 <= p2.wrapping_add(p4) {
                if p7 != 9 {
                    if p5 <= 33 {
                        if p3 != p3 {
                            if p7 >= (32 as u64).wrapping_add(p4) {
                                if p7 >= (59 as u64).wrapping_mul(p1) {
                                    if p6 > (27 as u64).wrapping_add(p1) {
                                        if p6 > p2 {
                                            panic!("AssertionFailed: 17"); // bug
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
        if p5 > p1 {
            if p0 == p4.wrapping_mul(p7) {
                if p6 < p0.wrapping_add(p2) {
                    if p3 >= (57 as u64).wrapping_mul(p1) {
                        if p1 >= p6.wrapping_mul(p1) {
                            if p7 <= 64 {
                                if p2 < p6.wrapping_add(p0) {
                                    if p0 < p5 {
                                        if p1 > p5.wrapping_add(p4) {
                                            if p2 > p0 {
                                                if p0 >= p4.wrapping_mul(p3) {
                                                    if p1 < p1.wrapping_add(p5) {
                                                        if p1 < (48 as u64).wrapping_mul(p7) {
                                                            if p4 == (34 as u64).wrapping_mul(p5) {
                                                                panic!("AssertionFailed: 18");
                                                                // bug
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
        return Ok(18);
    }
    if x == 2 && y == 5 {
        msg!("Wall: {}", 19);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 2 && y == 6 {
        if p7 <= 22 {
            if p7 == p0.wrapping_mul(p4) {
                if p4 <= 8 {
                    if p6 <= p5 {
                        if p1 > p5.wrapping_mul(p7) {
                            if p7 < p7.wrapping_mul(p5) {
                                if p7 <= (3 as u64).wrapping_mul(p5) {
                                    if p3 <= p5 {
                                        if p6 >= (13 as u64).wrapping_mul(p5) {
                                            panic!("AssertionFailed: 20"); // bug
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
        msg!("Wall: {}", 21);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 3 && y == 1 {
        if p6 != p6.wrapping_mul(p1) {
            if p2 < (36 as u64).wrapping_mul(p3) {
                if p6 > p4.wrapping_add(p4) {
                    if p3 >= p0 {
                        if p0 == p4.wrapping_add(p1) {
                            if p2 <= (31 as u64).wrapping_mul(p2) {
                                if p7 < (55 as u64).wrapping_add(p2) {
                                    if p7 != p7 {
                                        if p6 < p6 {
                                            if p1 < (13 as u64).wrapping_mul(p2) {
                                                if p3 > p6 {
                                                    if p5 != p3.wrapping_add(p1) {
                                                        if p4 != (51 as u64).wrapping_add(p6) {
                                                            if p1 != p3.wrapping_mul(p3) {
                                                                if p2 < p3 {
                                                                    panic!("AssertionFailed: 22");
                                                                    // bug
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
        return Ok(22);
    }
    if x == 3 && y == 2 {
        if p6 > p7.wrapping_mul(p0) {
            if p2 <= p6.wrapping_add(p4) {
                if p2 == p7 {
                    if p0 > p1 {
                        if p3 > (28 as u64).wrapping_add(p0) {
                            if p4 <= (36 as u64).wrapping_mul(p0) {
                                if p7 > p0.wrapping_add(p4) {
                                    if p5 >= p0.wrapping_add(p3) {
                                        panic!("AssertionFailed: 23"); // bug
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
        if p4 > 60 {
            if p7 >= (18 as u64).wrapping_mul(p3) {
                if p5 >= (49 as u64).wrapping_mul(p4) {
                    if p3 <= p0 {
                        if p2 != p2.wrapping_add(p5) {
                            if p0 <= (11 as u64).wrapping_mul(p3) {
                                if p2 >= (18 as u64).wrapping_mul(p6) {
                                    if p3 != p5 {
                                        if p1 <= (42 as u64).wrapping_add(p7) {
                                            if p5 == (21 as u64).wrapping_add(p5) {
                                                if p5 >= (49 as u64).wrapping_add(p4) {
                                                    panic!("AssertionFailed: 24");
                                                    // bug
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
        if p5 > p0.wrapping_mul(p4) {
            if p3 > p4 {
                if p0 != p5.wrapping_mul(p1) {
                    if p3 < 7 {
                        if p7 != (11 as u64).wrapping_add(p1) {
                            if p7 != p3 {
                                panic!("AssertionFailed: 25"); // bug
                            }
                        }
                    }
                }
            }
        }
        return Ok(25);
    }
    if x == 3 && y == 5 {
        if p5 < 37 {
            if p3 <= p5.wrapping_mul(p7) {
                if p2 == 36 {
                    if p6 >= 6 {
                        if p6 != p2.wrapping_mul(p5) {
                            if p5 > p4 {
                                if p1 < 18 {
                                    if p4 > p6 {
                                        if p5 <= p7.wrapping_add(p0) {
                                            if p5 != p1 {
                                                if p0 != (42 as u64).wrapping_add(p1) {
                                                    if p0 != p5.wrapping_add(p6) {
                                                        if p6 <= p3.wrapping_mul(p5) {
                                                            if p6 != (12 as u64).wrapping_mul(p6) {
                                                                if p2 > p4.wrapping_mul(p4) {
                                                                    if p6
                                                                        < (23 as u64)
                                                                            .wrapping_mul(p6)
                                                                    {
                                                                        panic!(
                                                                            "AssertionFailed: 26"
                                                                        ); // bug
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
        return Ok(26);
    }
    if x == 3 && y == 6 {
        if p6 == (12 as u64).wrapping_add(p3) {
            if p1 >= (52 as u64).wrapping_mul(p6) {
                if p1 < (20 as u64).wrapping_add(p0) {
                    if p6 > 5 {
                        if p3 <= 25 {
                            if p4 >= p2.wrapping_mul(p2) {
                                if p4 != (16 as u64).wrapping_mul(p3) {
                                    if p6 > (27 as u64).wrapping_mul(p7) {
                                        if p3 > (47 as u64).wrapping_mul(p1) {
                                            if p2 != (23 as u64).wrapping_mul(p5) {
                                                if p5 > (50 as u64).wrapping_mul(p0) {
                                                    if p1 < (52 as u64).wrapping_mul(p6) {
                                                        if p0 != p1.wrapping_mul(p6) {
                                                            if p4 == 19 {
                                                                panic!("AssertionFailed: 27");
                                                                // bug
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
        return Ok(27);
    }
    if x == 4 && y == 0 {
        msg!("Wall: {}", 28);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 4 && y == 1 {
        if p2 <= (52 as u64).wrapping_add(p7) {
            if p6 >= p2.wrapping_add(p5) {
                if p4 <= p0 {
                    if p7 > p2.wrapping_add(p7) {
                        if p4 <= 52 {
                            if p3 >= (33 as u64).wrapping_mul(p0) {
                                if p2 >= p4.wrapping_mul(p3) {
                                    if p2 < p5 {
                                        if p1 == p2.wrapping_mul(p6) {
                                            if p5 != p7 {
                                                panic!("AssertionFailed: 29"); // bug
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
        return Ok(29);
    }
    if x == 4 && y == 2 {
        if p1 <= p0.wrapping_add(p5) {
            if p0 > (58 as u64).wrapping_mul(p2) {
                if p5 < p2 {
                    if p6 > (42 as u64).wrapping_mul(p4) {
                        if p3 == (54 as u64).wrapping_mul(p0) {
                            if p1 >= p1.wrapping_add(p1) {
                                if p5 >= p6 {
                                    if p6 != (39 as u64).wrapping_add(p1) {
                                        if p4 == (30 as u64).wrapping_add(p2) {
                                            panic!("AssertionFailed: 30"); // bug
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
        if p2 < p4 {
            if p2 >= p0.wrapping_mul(p2) {
                if p2 >= p0 {
                    if p6 == 19 {
                        if p5 < (41 as u64).wrapping_add(p7) {
                            if p3 < (23 as u64).wrapping_add(p7) {
                                if p6 == p4.wrapping_mul(p6) {
                                    if p1 > (51 as u64).wrapping_add(p6) {
                                        if p4 < p4.wrapping_mul(p4) {
                                            if p1 >= p0 {
                                                if p1 != 49 {
                                                    if p1 != (23 as u64).wrapping_mul(p1) {
                                                        if p7 < p7 {
                                                            if p6 <= 26 {
                                                                if p6 >= 25 {
                                                                    if p3 > p5.wrapping_add(p3) {
                                                                        panic!(
                                                                            "AssertionFailed: 31"
                                                                        ); // bug
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
        return Ok(31);
    }
    if x == 4 && y == 4 {
        if p7 >= p0.wrapping_add(p5) {
            if p5 == (26 as u64).wrapping_mul(p0) {
                if p7 == p4.wrapping_mul(p0) {
                    if p0 <= p0 {
                        if p5 > p2.wrapping_mul(p5) {
                            if p1 <= (28 as u64).wrapping_mul(p4) {
                                if p2 > p4.wrapping_add(p0) {
                                    if p4 <= 36 {
                                        if p3 > p0 {
                                            if p6 > p6.wrapping_mul(p7) {
                                                if p7 >= p3.wrapping_add(p3) {
                                                    if p4 > p3 {
                                                        panic!("AssertionFailed: 32");
                                                        // bug
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
        return Ok(32);
    }
    if x == 4 && y == 5 {
        if p4 <= p7 {
            if p3 != p6.wrapping_mul(p3) {
                if p2 == (32 as u64).wrapping_mul(p1) {
                    if p5 > (25 as u64).wrapping_mul(p1) {
                        if p7 < (45 as u64).wrapping_mul(p7) {
                            if p7 >= p4.wrapping_add(p3) {
                                panic!("AssertionFailed: 33"); // bug
                            }
                        }
                    }
                }
            }
        }
        return Ok(33);
    }
    if x == 4 && y == 6 {
        if p5 >= 32 {
            if p5 <= 26 {
                if p0 != p3.wrapping_add(p3) {
                    if p6 == (12 as u64).wrapping_add(p4) {
                        if p2 != p6 {
                            if p1 < p1 {
                                if p1 < 59 {
                                    if p1 != (63 as u64).wrapping_add(p5) {
                                        if p1 <= p5.wrapping_mul(p2) {
                                            if p0 <= 5 {
                                                panic!("AssertionFailed: 34"); // bug
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
        if p4 == p6 {
            if p7 <= (40 as u64).wrapping_add(p4) {
                if p7 < 17 {
                    if p4 == (59 as u64).wrapping_mul(p6) {
                        if p0 == (29 as u64).wrapping_add(p0) {
                            if p6 != (40 as u64).wrapping_mul(p7) {
                                if p5 != (34 as u64).wrapping_mul(p1) {
                                    if p4 == 5 {
                                        if p7 >= 11 {
                                            if p0 == p6.wrapping_mul(p4) {
                                                if p4 >= p7 {
                                                    if p0 <= (10 as u64).wrapping_add(p7) {
                                                        if p5 <= 41 {
                                                            panic!("AssertionFailed: 35");
                                                            // bug
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
        return Ok(35);
    }
    if x == 5 && y == 1 {
        if p0 <= (19 as u64).wrapping_mul(p3) {
            if p1 >= p2 {
                if p7 == p7 {
                    if p0 >= (47 as u64).wrapping_mul(p0) {
                        if p3 >= (27 as u64).wrapping_add(p3) {
                            if p2 <= p3.wrapping_mul(p0) {
                                panic!("AssertionFailed: 36"); // bug
                            }
                        }
                    }
                }
            }
        }
        return Ok(36);
    }
    if x == 5 && y == 2 {
        if p5 != 28 {
            if p1 > p4.wrapping_mul(p6) {
                if p1 != 4 {
                    if p3 < (34 as u64).wrapping_add(p7) {
                        if p6 == (21 as u64).wrapping_add(p2) {
                            if p1 == p5.wrapping_mul(p6) {
                                if p7 <= (8 as u64).wrapping_mul(p0) {
                                    if p0 == p7.wrapping_add(p0) {
                                        if p6 == p1.wrapping_add(p2) {
                                            if p1 == (6 as u64).wrapping_add(p4) {
                                                if p2 == (25 as u64).wrapping_add(p3) {
                                                    if p4 < (58 as u64).wrapping_mul(p3) {
                                                        panic!("AssertionFailed: 37");
                                                        // bug
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
        if p3 <= p1.wrapping_mul(p4) {
            if p4 != p5.wrapping_add(p1) {
                if p7 > (40 as u64).wrapping_add(p5) {
                    if p4 == 19 {
                        if p2 == p6.wrapping_add(p7) {
                            if p0 > (0 as u64).wrapping_mul(p3) {
                                if p2 >= (25 as u64).wrapping_add(p2) {
                                    if p4 > p7 {
                                        if p1 < p5 {
                                            if p0 != (4 as u64).wrapping_mul(p7) {
                                                if p4 < (19 as u64).wrapping_add(p0) {
                                                    if p5 < p0.wrapping_add(p5) {
                                                        if p3 == (0 as u64).wrapping_add(p7) {
                                                            if p2 <= (6 as u64).wrapping_add(p1) {
                                                                panic!("AssertionFailed: 38");
                                                                // bug
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
        return Ok(38);
    }
    if x == 5 && y == 4 {
        if p6 == (42 as u64).wrapping_mul(p0) {
            if p0 == p0.wrapping_mul(p2) {
                if p3 < (25 as u64).wrapping_add(p3) {
                    if p4 == p6.wrapping_add(p5) {
                        if p6 <= 53 {
                            if p3 <= p5 {
                                if p2 > (54 as u64).wrapping_add(p5) {
                                    if p0 <= p5.wrapping_mul(p2) {
                                        if p0 <= p6 {
                                            panic!("AssertionFailed: 39"); // bug
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(39);
    }
    if x == 5 && y == 5 {
        if p1 != (2 as u64).wrapping_mul(p5) {
            if p0 >= p4.wrapping_mul(p3) {
                if p2 <= (5 as u64).wrapping_add(p2) {
                    if p1 > (30 as u64).wrapping_add(p0) {
                        if p2 <= (25 as u64).wrapping_add(p0) {
                            if p3 < p6 {
                                if p1 <= p1.wrapping_add(p1) {
                                    if p6 > (0 as u64).wrapping_add(p3) {
                                        if p0 >= (46 as u64).wrapping_mul(p4) {
                                            if p3 >= (2 as u64).wrapping_mul(p0) {
                                                panic!("AssertionFailed: 40"); // bug
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
        return Ok(40);
    }
    if x == 5 && y == 6 {
        if p3 != (14 as u64).wrapping_mul(p0) {
            if p7 != p4 {
                if p0 == 63 {
                    panic!("AssertionFailed: 41"); // bug
                }
            }
        }
        return Ok(41);
    }
    if x == 6 && y == 0 {
        if p0 < p0.wrapping_mul(p1) {
            if p5 < p5 {
                if p6 != p4 {
                    if p3 > p7 {
                        if p5 != p3 {
                            if p1 >= 54 {
                                if p0 < (30 as u64).wrapping_mul(p2) {
                                    if p2 != p0 {
                                        if p7 < (53 as u64).wrapping_mul(p0) {
                                            if p6 >= p2 {
                                                if p4 <= p1 {
                                                    panic!("AssertionFailed: 42");
                                                    // bug
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
        return Ok(42);
    }
    if x == 6 && y == 1 {
        if p1 <= p1.wrapping_add(p3) {
            if p4 < (28 as u64).wrapping_add(p0) {
                if p2 > p7 {
                    if p3 < (39 as u64).wrapping_add(p4) {
                        if p5 < p4.wrapping_mul(p1) {
                            if p2 < p7.wrapping_add(p6) {
                                if p0 < p4.wrapping_mul(p2) {
                                    if p4 >= (43 as u64).wrapping_add(p4) {
                                        if p4 <= p3.wrapping_add(p4) {
                                            if p2 < (12 as u64).wrapping_add(p7) {
                                                if p7 > p1.wrapping_mul(p4) {
                                                    if p3 >= p2 {
                                                        panic!("AssertionFailed: 43");
                                                        // bug
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
        return Ok(43);
    }
    if x == 6 && y == 2 {
        if p0 <= (14 as u64).wrapping_mul(p1) {
            if p0 < (9 as u64).wrapping_mul(p2) {
                if p7 > (42 as u64).wrapping_mul(p5) {
                    if p1 > (9 as u64).wrapping_add(p5) {
                        if p0 <= 28 {
                            if p4 > p0.wrapping_add(p7) {
                                if p1 > (9 as u64).wrapping_add(p1) {
                                    if p5 > (3 as u64).wrapping_add(p0) {
                                        if p3 < 21 {
                                            if p1 <= (52 as u64).wrapping_mul(p0) {
                                                if p0 < p5.wrapping_add(p3) {
                                                    if p2 >= 21 {
                                                        panic!("AssertionFailed: 44");
                                                        // bug
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
        return Ok(44);
    }
    if x == 6 && y == 3 {
        if p1 < p7 {
            if p0 == p6 {
                if p3 > p5.wrapping_mul(p5) {
                    if p1 > (2 as u64).wrapping_add(p6) {
                        if p3 == (28 as u64).wrapping_add(p2) {
                            if p7 <= p6.wrapping_mul(p1) {
                                if p2 > (63 as u64).wrapping_mul(p0) {
                                    if p4 != p6.wrapping_mul(p0) {
                                        if p1 == (33 as u64).wrapping_add(p2) {
                                            if p1 != 39 {
                                                if p7 == 55 {
                                                    if p7 > (17 as u64).wrapping_mul(p2) {
                                                        if p3 >= 2 {
                                                            if p7 < p6.wrapping_mul(p1) {
                                                                if p2 <= 13 {
                                                                    if p1
                                                                        > (24 as u64)
                                                                            .wrapping_mul(p2)
                                                                    {
                                                                        panic!(
                                                                            "AssertionFailed: 45"
                                                                        ); // bug
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
        msg!("Wall: {}", 46);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 6 && y == 5 {
        if p2 <= 28 {
            if p1 >= (36 as u64).wrapping_add(p4) {
                if p5 == (31 as u64).wrapping_mul(p2) {
                    if p6 <= p5 {
                        if p2 < p4 {
                            if p7 != (25 as u64).wrapping_add(p5) {
                                panic!("AssertionFailed: 47"); // bug
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
