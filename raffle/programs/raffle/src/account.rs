use anchor_lang::prelude::*;
use std::clone::Clone;
use std::result::Result;

use crate::constants::*;
use crate::error::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub super_admin: Pubkey, // 32
}

#[account(zero_copy)]
pub struct RafflePool {
    // 72+64+32*5000+48*50 = 66552
    pub creator: Pubkey,                    //32
    pub nft_mint: Pubkey,                   //32
    pub count: u64,                         //8
    pub winner_count: u64,                  //8
    pub no_repeat: u64,                     //8
    pub max_entrants: u64,                  //8
    pub start_timestamp: i64,               //8
    pub end_timestamp: i64,                 //8
    pub ticket_price_booga: u64,            //8
    pub ticket_price_zion: u64,             //8
    pub ticket_price_sol: u64,              //8
    pub whitelisted: u64,                   //8
    pub claimed_winner: [u64; MAX_WINNERS], //50*8
    pub indexes: [u64; MAX_WINNERS],        //50*8
    pub winner: [Pubkey; MAX_WINNERS],      //32*50
    pub entrants: [Pubkey; MAX_ENTRANTS],   //32*2000
}

impl Default for RafflePool {
    #[inline]
    fn default() -> RafflePool {
        RafflePool {
            creator: Pubkey::default(),
            nft_mint: Pubkey::default(),
            count: 0,
            winner_count: 0,
            no_repeat: 0,
            max_entrants: 0,
            start_timestamp: 0,
            end_timestamp: 0,
            ticket_price_booga: 0,
            ticket_price_zion: 0,
            ticket_price_sol: 0,
            whitelisted: 0,
            claimed_winner: [0; MAX_WINNERS],
            indexes: [0; MAX_WINNERS],
            winner: [Pubkey::default(); MAX_WINNERS],
            entrants: [Pubkey::default(); MAX_ENTRANTS],
        }
    }
}
impl RafflePool {
    pub fn append(&mut self, buyer: Pubkey) {
        self.entrants[self.count as usize] = buyer;
        self.count += 1;
    }
}
