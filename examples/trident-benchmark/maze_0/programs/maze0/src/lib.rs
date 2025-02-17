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
        // msg!("We are inside start");
        // msg!("Data len: {:#?}", ctx.accounts.state.to_account_info());
        // eprintln!("Data len: {:#?}", ctx.accounts.state.to_account_info());
        // println!("Data len: {:#?}", ctx.accounts.state.to_account_info());

        let state = &mut ctx.accounts.state;

        // let rent = Rent::get()?;
        state.x = 0;
        state.y = 0;

        // msg!("We are inside end");
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
        if p2 != p2 {
            if p0 < p5 {
                if p0 != 46 {
                    if p4 != 27 {
                        if p2 < (61 as u64).wrapping_mul(p3) {
                            if p2 < 49 {
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
        if p0 >= p4 {
            if p4 > p4.wrapping_mul(p6) {
                if p0 > (27 as u64).wrapping_mul(p2) {
                    if p4 > 20 {
                        if p7 < 49 {
                            if p1 == (7 as u64).wrapping_add(p2) {
                                if p3 <= 33 {
                                    if p4 >= (22 as u64).wrapping_add(p3) {
                                        if p2 <= (12 as u64).wrapping_mul(p7) {
                                            if p1 < p7 {
                                                if p4 > p2.wrapping_add(p5) {
                                                    panic!("AssertionFailed: 2");
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
        return Ok(2);
    }
    if x == 0 && y == 3 {
        if p7 <= p6.wrapping_add(p6) {
            if p3 <= (64 as u64).wrapping_mul(p2) {
                if p4 <= 34 {
                    if p0 >= (28 as u64).wrapping_mul(p2) {
                        if p4 > 37 {
                            if p5 >= p5.wrapping_mul(p6) {
                                if p7 == (20 as u64).wrapping_mul(p6) {
                                    if p2 >= 48 {
                                        if p3 >= p0.wrapping_add(p0) {
                                            if p3 <= 4 {
                                                if p7 < p7.wrapping_add(p5) {
                                                    if p1 != 58 {
                                                        if p4 >= 47 {
                                                            panic!("AssertionFailed: 3");
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
        return Ok(3);
    }
    if x == 0 && y == 4 {
        if p2 != 37 {
            if p1 <= p1.wrapping_add(p5) {
                if p1 > 2 {
                    if p0 == (13 as u64).wrapping_mul(p3) {
                        if p2 == p2.wrapping_mul(p7) {
                            if p4 == 28 {
                                if p3 > 31 {
                                    if p4 < (40 as u64).wrapping_mul(p4) {
                                        panic!("AssertionFailed: 4"); //bug
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return Ok(4);
    }
    if x == 0 && y == 5 {
        if p6 != (2 as u64).wrapping_add(p1) {
            if p4 == p5.wrapping_mul(p5) {
                panic!("AssertionFailed: 5"); //bug
            }
        }
        return Ok(5);
    }
    if x == 0 && y == 6 {
        if p6 >= (39 as u64).wrapping_mul(p5) {
            if p4 >= (55 as u64).wrapping_mul(p4) {
                if p4 <= p6.wrapping_mul(p6) {
                    if p2 == (12 as u64).wrapping_add(p4) {
                        if p0 <= (18 as u64).wrapping_mul(p2) {
                            if p1 >= 57 {
                                if p6 >= 41 {
                                    panic!("AssertionFailed: 6"); //bug
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
        if p7 <= p6.wrapping_add(p5) {
            if p0 >= (62 as u64).wrapping_add(p2) {
                if p2 == p3.wrapping_mul(p3) {
                    if p0 >= 24 {
                        if p2 >= p7.wrapping_add(p6) {
                            if p1 >= p2 {
                                if p1 == p4.wrapping_mul(p3) {
                                    if p0 < (54 as u64).wrapping_mul(p2) {
                                        panic!("AssertionFailed: 8"); //bug
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
        if p0 < p4 {
            if p7 <= p7 {
                if p0 < p0.wrapping_mul(p4) {
                    if p7 >= p2.wrapping_add(p1) {
                        if p6 >= p6 {
                            if p4 > p7.wrapping_mul(p4) {
                                if p3 < 29 {
                                    panic!("AssertionFailed: 9"); //bug
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
        if p3 != 41 {
            if p7 == (46 as u64).wrapping_add(p4) {
                if p6 == p1 {
                    if p3 >= p4.wrapping_mul(p6) {
                        if p5 < p7.wrapping_mul(p5) {
                            if p4 == p3 {
                                if p5 > p4.wrapping_add(p3) {
                                    if p3 != (53 as u64).wrapping_add(p6) {
                                        if p5 >= p2.wrapping_mul(p7) {
                                            if p2 == 21 {
                                                if p6 > (43 as u64).wrapping_mul(p3) {
                                                    if p1 == p7.wrapping_mul(p4) {
                                                        if p5 <= (8 as u64).wrapping_mul(p0) {
                                                            if p7 > 17 {
                                                                if p4 < p7.wrapping_mul(p1) {
                                                                    if p5 > p1.wrapping_mul(p0) {
                                                                        panic!(
                                                                            "AssertionFailed: 10"
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
        return Ok(10);
    }
    if x == 1 && y == 4 {
        if p6 < p2.wrapping_mul(p6) {
            if p3 == p3.wrapping_mul(p2) {
                if p2 != p4.wrapping_mul(p4) {
                    if p3 > (61 as u64).wrapping_add(p4) {
                        if p7 > p7.wrapping_mul(p4) {
                            if p1 < (9 as u64).wrapping_mul(p0) {
                                if p1 != p2.wrapping_mul(p7) {
                                    panic!("AssertionFailed: 11"); //bug
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
        msg!("Wall: {}", 12);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 1 && y == 6 {
        if p0 <= (51 as u64).wrapping_add(p3) {
            if p4 != (31 as u64).wrapping_add(p7) {
                if p5 < p2.wrapping_mul(p4) {
                    if p0 > 46 {
                        if p6 == 11 {
                            if p0 == 50 {
                                if p3 <= p0 {
                                    if p5 < 41 {
                                        panic!("AssertionFailed: 13"); //bug
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
        if p5 >= p2 {
            if p3 <= p0.wrapping_mul(p4) {
                if p2 > p2 {
                    if p7 <= p0.wrapping_add(p6) {
                        if p6 <= (26 as u64).wrapping_add(p1) {
                            if p6 != (21 as u64).wrapping_mul(p6) {
                                if p5 != (42 as u64).wrapping_mul(p0) {
                                    if p1 < p6.wrapping_add(p1) {
                                        if p1 <= 25 {
                                            if p5 <= (0 as u64).wrapping_mul(p6) {
                                                if p3 == p2.wrapping_add(p2) {
                                                    if p3 < p7 {
                                                        panic!("AssertionFailed: 14");
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
        return Ok(14);
    }
    if x == 2 && y == 1 {
        if p3 != 6 {
            if p3 < p3 {
                if p3 < p4.wrapping_add(p7) {
                    if p5 < p1.wrapping_add(p7) {
                        if p4 > (21 as u64).wrapping_add(p0) {
                            if p0 <= (60 as u64).wrapping_add(p0) {
                                if p7 < (39 as u64).wrapping_mul(p3) {
                                    if p2 <= 19 {
                                        if p7 != (26 as u64).wrapping_add(p0) {
                                            if p4 > 15 {
                                                if p0 != (47 as u64).wrapping_add(p1) {
                                                    if p2 != p2.wrapping_add(p0) {
                                                        if p6 >= p5.wrapping_add(p6) {
                                                            if p0 == p0.wrapping_mul(p2) {
                                                                if p6 < 31 {
                                                                    if p4 <= p7.wrapping_mul(p5) {
                                                                        panic!(
                                                                            "AssertionFailed: 15"
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
        return Ok(15);
    }
    if x == 2 && y == 2 {
        if p5 != 10 {
            if p5 != p0 {
                if p4 >= p2.wrapping_mul(p1) {
                    if p3 > (31 as u64).wrapping_add(p5) {
                        panic!("AssertionFailed: 16"); //bug
                    }
                }
            }
        }
        return Ok(16);
    }
    if x == 2 && y == 3 {
        if p6 == 10 {
            if p5 == (50 as u64).wrapping_mul(p5) {
                if p0 > 17 {
                    panic!("AssertionFailed: 17"); //bug
                }
            }
        }
        return Ok(17);
    }
    if x == 2 && y == 4 {
        if p5 >= p5.wrapping_mul(p7) {
            if p0 < p1 {
                if p2 == p3.wrapping_add(p7) {
                    panic!("AssertionFailed: 18"); //bug
                }
            }
        }
        return Ok(18);
    }
    if x == 2 && y == 5 {
        if p4 > p2 {
            if p6 > 40 {
                if p1 != p1.wrapping_mul(p1) {
                    if p4 > p7.wrapping_add(p5) {
                        panic!("AssertionFailed: 19"); //bug
                    }
                }
            }
        }
        return Ok(19);
    }
    if x == 2 && y == 6 {
        msg!("Wall: {}", 20);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 3 && y == 0 {
        if p3 > (3 as u64).wrapping_mul(p0) {
            if p1 != 13 {
                if p1 > 45 {
                    if p5 > (39 as u64).wrapping_add(p0) {
                        if p7 == p2.wrapping_add(p0) {
                            if p4 >= (16 as u64).wrapping_mul(p1) {
                                if p4 == 38 {
                                    if p0 >= (52 as u64).wrapping_mul(p0) {
                                        if p2 == p6.wrapping_mul(p5) {
                                            if p2 < (2 as u64).wrapping_add(p1) {
                                                if p1 <= p1 {
                                                    if p2 > p7 {
                                                        panic!("AssertionFailed: 21");
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
        return Ok(21);
    }
    if x == 3 && y == 1 {
        msg!("Wall: {}", 22);
        return Err(MazeError::Wall.into()); // wall
    }
    if x == 3 && y == 2 {
        if p3 > p0.wrapping_add(p6) {
            if p5 != p6.wrapping_add(p4) {
                if p2 >= p0.wrapping_add(p6) {
                    if p4 != p1.wrapping_mul(p6) {
                        if p6 > p3.wrapping_add(p4) {
                            if p4 > (29 as u64).wrapping_mul(p2) {
                                if p7 != p0.wrapping_add(p7) {
                                    if p3 != p6.wrapping_mul(p5) {
                                        if p4 != p3.wrapping_mul(p1) {
                                            if p0 == 55 {
                                                if p2 >= (23 as u64).wrapping_mul(p5) {
                                                    if p1 <= p1 {
                                                        if p2 > p4.wrapping_mul(p7) {
                                                            if p1 <= (45 as u64).wrapping_mul(p5) {
                                                                if p4 > p7.wrapping_add(p7) {
                                                                    if p3
                                                                        > (54 as u64)
                                                                            .wrapping_mul(p4)
                                                                    {
                                                                        panic!(
                                                                            "AssertionFailed: 23"
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
        return Ok(23);
    }
    if x == 3 && y == 3 {
        if p0 > p2 {
            if p6 != p6 {
                if p5 != p2.wrapping_add(p2) {
                    panic!("AssertionFailed: 24"); //bug
                }
            }
        }
        return Ok(24);
    }
    if x == 3 && y == 4 {
        if p2 >= 34 {
            if p3 <= 9 {
                if p1 >= p2.wrapping_mul(p4) {
                    if p1 == p7 {
                        if p0 > (57 as u64).wrapping_add(p6) {
                            if p1 <= (53 as u64).wrapping_add(p4) {
                                if p2 >= (25 as u64).wrapping_mul(p0) {
                                    if p6 < p5.wrapping_add(p6) {
                                        if p3 >= (48 as u64).wrapping_mul(p6) {
                                            if p7 <= (23 as u64).wrapping_add(p3) {
                                                panic!("AssertionFailed: 25"); //bug
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
        if p0 > p1.wrapping_add(p7) {
            if p0 == p7 {
                if p2 != p6.wrapping_add(p4) {
                    if p4 >= 47 {
                        if p5 >= p5.wrapping_add(p4) {
                            if p3 != 28 {
                                if p7 == 36 {
                                    if p5 > p4.wrapping_mul(p4) {
                                        panic!("AssertionFailed: 26"); //bug
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
        if p7 < 1 {
            if p1 > (8 as u64).wrapping_mul(p6) {
                if p7 <= (31 as u64).wrapping_mul(p6) {
                    panic!("AssertionFailed: 27"); //bug
                }
            }
        }
        return Ok(27);
    }
    if x == 4 && y == 0 {
        if p3 >= (9 as u64).wrapping_mul(p7) {
            if p5 < 8 {
                if p4 != p1 {
                    if p6 >= (3 as u64).wrapping_mul(p2) {
                        if p0 >= p5.wrapping_add(p3) {
                            if p2 < (63 as u64).wrapping_add(p7) {
                                if p6 <= (3 as u64).wrapping_mul(p4) {
                                    if p2 <= 46 {
                                        if p0 != (42 as u64).wrapping_mul(p1) {
                                            if p1 == p5 {
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
        if p1 > 63 {
            if p0 == (35 as u64).wrapping_mul(p2) {
                if p0 != p2.wrapping_mul(p6) {
                    panic!("AssertionFailed: 29"); //bug
                }
            }
        }
        return Ok(29);
    }
    if x == 4 && y == 2 {
        if p4 <= (31 as u64).wrapping_mul(p4) {
            if p5 < p5.wrapping_mul(p5) {
                if p5 > (41 as u64).wrapping_add(p6) {
                    if p3 >= (45 as u64).wrapping_add(p4) {
                        if p6 <= (8 as u64).wrapping_mul(p7) {
                            if p0 < 6 {
                                if p2 == p0.wrapping_mul(p3) {
                                    panic!("AssertionFailed: 30"); //bug
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
        if p0 == (49 as u64).wrapping_add(p5) {
            if p2 >= 30 {
                if p6 > (50 as u64).wrapping_mul(p0) {
                    if p3 <= (13 as u64).wrapping_mul(p1) {
                        if p5 > p6.wrapping_add(p2) {
                            if p1 <= p5 {
                                if p4 <= p5 {
                                    if p5 > (49 as u64).wrapping_mul(p7) {
                                        if p2 > p4.wrapping_mul(p3) {
                                            if p4 != 33 {
                                                if p2 != (14 as u64).wrapping_add(p5) {
                                                    if p5 <= 25 {
                                                        if p4 < (52 as u64).wrapping_add(p5) {
                                                            if p1 >= p2 {
                                                                panic!("AssertionFailed: 31");
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
        return Ok(31);
    }
    if x == 4 && y == 4 {
        if p2 > 39 {
            if p2 >= p5.wrapping_add(p1) {
                if p6 >= p2.wrapping_mul(p6) {
                    if p7 > 5 {
                        if p0 == 34 {
                            if p1 >= (56 as u64).wrapping_add(p3) {
                                if p3 == p4 {
                                    if p2 > (4 as u64).wrapping_mul(p1) {
                                        if p2 <= 18 {
                                            if p5 == (10 as u64).wrapping_add(p0) {
                                                if p7 <= p0.wrapping_mul(p2) {
                                                    if p3 < p4.wrapping_mul(p7) {
                                                        if p6 == 42 {
                                                            if p3 < p6.wrapping_mul(p7) {
                                                                if p5 != p1.wrapping_mul(p2) {
                                                                    panic!("AssertionFailed: 32");
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
        }
        return Ok(32);
    }
    if x == 4 && y == 5 {
        if p7 <= p4 {
            if p4 >= p7.wrapping_mul(p6) {
                if p5 <= p6 {
                    if p4 <= p1 {
                        if p2 != p2.wrapping_mul(p7) {
                            if p0 <= p4.wrapping_mul(p0) {
                                if p6 <= p1 {
                                    if p0 < p6.wrapping_add(p1) {
                                        if p7 <= p7 {
                                            if p0 >= (61 as u64).wrapping_mul(p4) {
                                                if p2 < p0.wrapping_mul(p3) {
                                                    if p1 == p0 {
                                                        if p3 == (59 as u64).wrapping_mul(p0) {
                                                            if p3 <= (16 as u64).wrapping_add(p6) {
                                                                if p4 == p2.wrapping_mul(p1) {
                                                                    if p7 < 5 {
                                                                        panic!(
                                                                            "AssertionFailed: 33"
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
        return Ok(33);
    }
    if x == 4 && y == 6 {
        if p4 <= (4 as u64).wrapping_add(p1) {
            if p6 < (63 as u64).wrapping_add(p3) {
                if p5 >= (30 as u64).wrapping_mul(p5) {
                    if p2 != (27 as u64).wrapping_mul(p6) {
                        panic!("AssertionFailed: 34"); //bug
                    }
                }
            }
        }
        return Ok(34);
    }
    if x == 5 && y == 0 {
        if p3 >= 55 {
            if p4 >= (52 as u64).wrapping_mul(p2) {
                if p0 != (56 as u64).wrapping_mul(p1) {
                    panic!("AssertionFailed: 35"); //bug
                }
            }
        }
        return Ok(35);
    }
    if x == 5 && y == 1 {
        if p1 > p6 {
            if p7 < (34 as u64).wrapping_mul(p4) {
                if p2 == p1 {
                    if p7 >= 32 {
                        if p1 == (38 as u64).wrapping_mul(p1) {
                            if p0 > (52 as u64).wrapping_mul(p7) {
                                if p0 > (38 as u64).wrapping_add(p5) {
                                    if p3 > 64 {
                                        if p3 > p6 {
                                            if p3 == p0 {
                                                if p1 > 42 {
                                                    if p6 <= p3 {
                                                        if p3 != (55 as u64).wrapping_mul(p0) {
                                                            panic!("AssertionFailed: 36");
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
        return Ok(36);
    }
    if x == 5 && y == 2 {
        if p4 == 26 {
            if p7 < p4 {
                if p6 > p2 {
                    if p6 > p5 {
                        if p2 > p0.wrapping_mul(p2) {
                            if p3 >= p5.wrapping_add(p7) {
                                if p6 < p3 {
                                    if p7 < 46 {
                                        if p2 < (44 as u64).wrapping_mul(p1) {
                                            if p7 != p0 {
                                                if p5 != 35 {
                                                    if p6 != p2.wrapping_add(p2) {
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
        if p2 >= (6 as u64).wrapping_mul(p4) {
            if p5 <= (6 as u64).wrapping_add(p1) {
                if p4 <= p6 {
                    if p5 < p5.wrapping_mul(p2) {
                        if p2 >= p4 {
                            if p3 == 43 {
                                if p4 <= (49 as u64).wrapping_add(p6) {
                                    if p3 <= p0.wrapping_mul(p3) {
                                        if p3 != (28 as u64).wrapping_add(p7) {
                                            if p2 < (20 as u64).wrapping_mul(p1) {
                                                if p2 != p6 {
                                                    panic!("AssertionFailed: 38");
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
        return Ok(38);
    }
    if x == 5 && y == 4 {
        if p5 < (59 as u64).wrapping_mul(p6) {
            if p5 <= 32 {
                if p3 < p4.wrapping_add(p1) {
                    if p5 == 31 {
                        if p7 == (36 as u64).wrapping_mul(p1) {
                            panic!("AssertionFailed: 39"); //bug
                        }
                    }
                }
            }
        }
        return Ok(39);
    }
    if x == 5 && y == 5 {
        if p5 > (41 as u64).wrapping_add(p3) {
            if p4 >= p5 {
                if p6 == (25 as u64).wrapping_add(p5) {
                    if p0 > p5.wrapping_mul(p3) {
                        if p7 == 7 {
                            if p3 >= 19 {
                                if p5 <= (61 as u64).wrapping_mul(p5) {
                                    if p7 < p1 {
                                        if p2 <= p3 {
                                            panic!("AssertionFailed: 40"); //bug
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
        if p7 != p2.wrapping_add(p3) {
            if p2 < 60 {
                if p0 < p4.wrapping_add(p0) {
                    if p3 != p4.wrapping_mul(p0) {
                        if p1 <= 33 {
                            if p1 >= p6.wrapping_add(p6) {
                                if p4 <= p2.wrapping_add(p0) {
                                    if p5 <= (47 as u64).wrapping_mul(p7) {
                                        if p5 != (12 as u64).wrapping_add(p6) {
                                            if p5 >= (52 as u64).wrapping_mul(p2) {
                                                if p0 <= p2.wrapping_mul(p5) {
                                                    if p3 >= p7 {
                                                        if p2 == p4 {
                                                            panic!("AssertionFailed: 41");
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
        return Ok(41);
    }
    if x == 6 && y == 0 {
        if p4 >= (34 as u64).wrapping_add(p7) {
            if p4 != p7.wrapping_add(p2) {
                if p7 <= p7.wrapping_mul(p3) {
                    panic!("AssertionFailed: 42"); //bug
                }
            }
        }
        return Ok(42);
    }
    if x == 6 && y == 1 {
        if p2 == p4.wrapping_add(p1) {
            if p5 == (12 as u64).wrapping_add(p3) {
                if p7 >= p5.wrapping_mul(p3) {
                    panic!("AssertionFailed: 43"); //bug
                }
            }
        }
        return Ok(43);
    }
    if x == 6 && y == 2 {
        if p5 == (3 as u64).wrapping_add(p5) {
            if p0 != p0.wrapping_mul(p1) {
                panic!("AssertionFailed: 44"); //bug
            }
        }
        return Ok(44);
    }
    if x == 6 && y == 3 {
        if p0 < 23 {
            if p2 >= p0 {
                if p3 < (50 as u64).wrapping_mul(p2) {
                    if p4 > p3.wrapping_add(p1) {
                        if p7 >= (52 as u64).wrapping_mul(p5) {
                            if p6 <= p1.wrapping_mul(p6) {
                                if p1 > p3.wrapping_add(p7) {
                                    if p6 < p5.wrapping_add(p1) {
                                        if p0 > (9 as u64).wrapping_add(p5) {
                                            if p6 < p0 {
                                                if p1 > p0.wrapping_add(p2) {
                                                    if p3 <= p0.wrapping_mul(p0) {
                                                        if p5 != p7 {
                                                            if p1 < p5.wrapping_add(p0) {
                                                                if p6 == p7.wrapping_add(p5) {
                                                                    panic!("AssertionFailed: 45");
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
        }
        return Ok(45);
    }
    if x == 6 && y == 4 {
        if p7 <= p1 {
            if p5 < p2 {
                if p2 != p3 {
                    if p0 > (39 as u64).wrapping_add(p7) {
                        if p3 <= p1.wrapping_mul(p6) {
                            if p2 > 34 {
                                if p4 == (26 as u64).wrapping_mul(p4) {
                                    if p3 >= p2 {
                                        if p5 >= (15 as u64).wrapping_mul(p3) {
                                            if p4 == p1.wrapping_add(p2) {
                                                if p1 > (0 as u64).wrapping_add(p5) {
                                                    if p4 <= p7.wrapping_mul(p1) {
                                                        if p7 >= 56 {
                                                            if p4 > 61 {
                                                                if p7 > p1.wrapping_mul(p6) {
                                                                    panic!("AssertionFailed: 46");
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
        }
        return Ok(46);
    }
    if x == 6 && y == 5 {
        if p0 > 18 {
            if p7 >= (59 as u64).wrapping_mul(p0) {
                if p0 >= 37 {
                    if p5 < (44 as u64).wrapping_mul(p3) {
                        if p2 < p3.wrapping_mul(p7) {
                            if p0 <= p3.wrapping_mul(p6) {
                                if p7 <= p1 {
                                    if p7 >= (8 as u64).wrapping_add(p5) {
                                        if p5 < p5.wrapping_mul(p3) {
                                            if p3 < (55 as u64).wrapping_add(p3) {
                                                if p4 < 41 {
                                                    if p0 >= (53 as u64).wrapping_add(p7) {
                                                        if p4 > (52 as u64).wrapping_mul(p1) {
                                                            if p2 == p0.wrapping_add(p7) {
                                                                if p6 > p0.wrapping_mul(p5) {
                                                                    if p6 <= p4.wrapping_add(p4) {
                                                                        panic!(
                                                                            "AssertionFailed: 47"
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
