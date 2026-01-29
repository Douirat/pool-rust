Ben</Douirat>
bendouirat
Invisible

Ben</Douirat> — 02/01/2026 11:17
i try and they don't respond
aotchoun

 — 02/01/2026 11:18
I just sent him a message.
I'll see if he replies.
Ben</Douirat> — 02/01/2026 11:19
okay after he accepts push it okay.
aotchoun

 — 02/01/2026 11:19
Okay no problem
aotchoun

 — 02/01/2026 12:00
Toujours pas de réponse
Ben</Douirat> — 02/01/2026 12:00
you have a call
we need to call him
or send him a message to his phone
at least
0682111361 here is his number
can you
aotchoun

 — 02/01/2026 12:03
Okay no problem
Ben</Douirat> — 02/01/2026 12:03
merci
Ben</Douirat> — 02/01/2026 12:13
0630570771
Ben</Douirat> — 03/01/2026 09:40
hello agiel
aotchoun

 — 03/01/2026 10:11
Salut Ben
Comment tu vas ?
Ben</Douirat> — 03/01/2026 10:35
are you comming
med amine is here
and we need to start audits right now
aotchoun

 — 03/01/2026 10:38
Okay, no problem, I'll be there in an hour.
Abderrahim is here?
Ben</Douirat> — 03/01/2026 10:39
please try to be here earlier one of the auditors wants to leave$
and the others are absents 
aotchoun

 — 03/01/2026 10:40
Okay.
Actually, I'm at the airport next door, so I'll leave the airport now and walk here.
Ben</Douirat> — 03/01/2026 10:46
just try to catch a taxi on the road. we have no other optionb
aotchoun

 — 03/01/2026 10:52
Okay, I'll go find a taxi.
aotchoun

 — 03/01/2026 11:08
I'm here
aotchoun

 — 04/01/2026 15:42
piscine-rust
Ben</Douirat> — 06/01/2026 17:33
   "checkpoint-01-rust": {
              "id": 100749,
              "name": "Checkpoint 01-rust",
              "type": "exam",
              "attrs": {
                "campus": "oujda",
Afficher plus
message.txt
18 Ko
aotchoun

 — 06/01/2026 17:35
https://github.com/kinoz01/rust-piscine/tree/main
GitHub
GitHub - kinoz01/rust-piscine
Contribute to kinoz01/rust-piscine development by creating an account on GitHub.
Contribute to kinoz01/rust-piscine development by creating an account on GitHub.
aotchoun

 — 12/01/2026 16:46
[package]
name = "boxing_todo"
version = "0.1.0"
edition = "2024"

[dependencies]
json = "0.12.4"
tempfile = "3.20.0"
Ben</Douirat> — 15/01/2026 16:20
https://prod.liveshare.vsengsaas.visualstudio.com/join?4A0E91EDB1B3A49960D1A63045CE47494DD0
Visual Studio Code for the Web
Build with Visual Studio Code, anywhere, anytime, entirely in your browser.
Ben</Douirat> — 15/01/2026 16:55
sequenceDiagram
    actor User
    participant Main as Main Loop (REPL)
    participant Input as Input Handler
    participant Parser as Command Parser
    participant Router as Command Router
Afficher plus
message.txt
7 Ko
Ben</Douirat> — 16/01/2026 11:33
hello agiel are you available
Ben</Douirat> — 16/01/2026 12:16
create your branch please to ease the merge cause i will work on another file
aotchoun

 — 16/01/2026 12:16
Okay pas de problème
aotchoun

 — 16/01/2026 13:19
J'ai ajouter la gestion des flags, j'ai modifié le fichier command.rs pour centraliser tout ce qui concerne les commandes dans ce fichier
aotchoun

 — 16/01/2026 17:20
C'est régler
je pars à 17h30
Ben</Douirat> — 17/01/2026 09:00
good work agiel
i''l be in the zone when you come.
aotchoun

 — 17/01/2026 14:47
Je viens d'ajouter le séparateur
aotchoun

 — 17/01/2026 14:56
Fait le pull du main avant de continuer à travailler
aotchoun

 — 17/01/2026 16:57
Attends que je fasse le merge
je vais faire le merge avec main maintenant
C'est bon j'ai fait le merge
Ben</Douirat> — 18/01/2026 13:32
good work agiel we will meet tomorrow and continuo the work togather.
Ben</Douirat> — 27/01/2026 08:05
agiel, logically i finished the ls and cd please work on the other functionalities if you can we need to finesh the logic of the app to focus on testing
aotchoun

 — 27/01/2026 09:21
Hello Ben,
Okay, no problem
Ben</Douirat> — 15:08
agiel send me the sales
aotchoun

 — 15:09
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        // Chercher le produit dans le store
        for (name, price) in &s.products {
            if name == &ele {
                self.items.push((ele.clone(), *price));
                break;
            }
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let free_items = prices.len() / 3;

        let total_discount: f32 = prices.iter().take(free_items).sum();

        let total_price: f32 = prices.iter().sum();

        let final_price = total_price - total_discount;

        let ratio = if total_price > 0.0 {
            final_price / total_price
        } else {
            1.0
        };

        let receipt: Vec<f32> = prices
            .iter()
            .map(|price| (price * ratio * 100.0).round() / 100.0)
            .collect();

        self.receipt = receipt.clone();

        receipt
    }
}
aotchoun

 — 16:36
Iterators
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            return None;
        }

        let current = self.v;
       
        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }

        Some(Collatz { v: current })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n <= 1 {
        return 0;
    }

    Collatz::new(n).count()
}
Roman_numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomanDigit {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
Afficher plus
lib.rs
3 Ko
slice to map
use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Eq + Hash, U>(
    keys: &'a [T],
    values: &'a [U],
Afficher plus
lib.rs
1 Ko
step iterator
use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
Afficher plus
lib.rs
1 Ko
﻿
aotchoun
agiel_oth

 
use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    finished: bool,
}

impl<T> StepIterator<T>
where
    T: Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            finished: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Copy + PartialOrd + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // Si on dépasse end → stop
        if self.current > self.end {
            self.finished = true;
            return None;
        }

        let value = self.current;
        self.current = self.current + self.step;

        Some(value)
    }
}
lib.rs
1 Ko