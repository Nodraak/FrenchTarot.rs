extern crate rand;
use crate::card::rand::Rng;

/// the Card interface
pub trait Card{
    /// Convert the card into human readable version
    fn to_readable_string(&self) -> String;
    /// Display the card into human readable version to stdout
    fn display(&self);
    /// indicate if a Card can be played in the Table
    fn can_play(&self) -> bool;
}

#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum CardColor
{
    Pique,
    Coeur,
    Carreau,
    Trefle,
}

#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum CardValue
{
    As,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Valet,
    Cavalier,
    Dame,
    Roi,
}

#[derive(Debug)]
pub struct ClassicCard
{
    pub value: CardValue,
    pub color: CardColor,
}

impl Card for ClassicCard
{
    /// Print a ClassicCard to human readable string
    ///
    /// # Examples
    ///
    /// ```
    /// use tarot_lib::card::ClassicCard;
    /// use tarot_lib::card::CardValue;
    /// use tarot_lib::card::CardColor;
    /// use tarot_lib::card::Card;
    ///
    /// let card = ClassicCard{value: CardValue::As, color: CardColor::Pique};
    /// let printable_card = card.to_readable_string();
    ///
    /// assert_eq!("As de Pique", printable_card);
    /// ```
    fn to_readable_string(&self) -> String {
        return format!("{:?} de {:?}", self.value, self.color);
    }

    /// Print the ClassicCard to stdout using to_readable_string fonction
    fn display(&self){
        println!("{}", self.to_readable_string());
    }

    /// indicate if a ClassicCard can be played on the table or not
    fn can_play(&self) -> bool {
        //TODO implement
        return true;
    }
}

#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum AtoutValue
{
    Excuse,
    Un,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Onze,
    Douze,
    Treize,
    Quatorze,
    Quinze,
    Seize,
    DixSept,
    DixHuit,
    DixNeuf,
    Vingt,
    VingtEtUn,
}

#[derive(Debug)]
pub struct AtoutCard
{
    pub value: AtoutValue,
}

impl Card for AtoutCard
{
    /// Print an AtoutCard to human readable string
    ///
    /// # Examples
    ///
    /// ```
    /// use tarot_lib::card::AtoutCard;
    /// use tarot_lib::card::AtoutValue;
    /// use tarot_lib::card::Card;
    ///
    /// let card = AtoutCard{value: AtoutValue::Un};
    /// let printable_card = card.to_readable_string();
    /// assert_eq!("Un d'Atout", printable_card);
    ///
    /// let card = AtoutCard{value: AtoutValue::Excuse};
    /// let printable_card = card.to_readable_string();
    /// assert_eq!("Excuse", printable_card);
    ///
    /// ```
    fn to_readable_string(&self) -> String {
        let ret: String;
        if AtoutValue::Excuse == self.value {
            ret = format!("Excuse");
        } else {
            ret = format!("{:?} d'Atout", self.value);
        }
        return ret;
    }

    /// Print the AtoutCard to stdout using to_readable_string fonction
    fn display(&self){
        println!("{}", self.to_readable_string());
    }

    /// indicate if a AtoutCard can be played on the table or not
    fn can_play(&self) -> bool {
        //TODO implement
        return true;
    }
}

/// A DeckCard is either a ClassicCard or an AtoutCard
type DeckCard = Box<dyn Card>;

/// A Trick is a set of unique DeckCards. Contains up to the number of players.
pub struct Trick {
    pub cards: Vec<DeckCard>,
}

/// A Deck is a set of unique DeckCards
pub struct Deck {
    pub cards: Vec<DeckCard>,
}

pub struct Table{
    pub deck_player1: Deck,
    pub deck_player2: Deck,
    pub deck_player3: Deck,
    pub deck_player4: Deck,
    pub deck_player5: Deck,
    pub deck_chien: Deck,
}

impl Table {
    pub fn new() -> Self {
        Table{
            deck_chien : Deck{cards: Vec::new()},
            deck_player1 : Deck{cards: Vec::new()},
            deck_player2 : Deck{cards: Vec::new()},
            deck_player3 : Deck{cards: Vec::new()},
            deck_player4 : Deck{cards: Vec::new()},
            deck_player5 : Deck{cards: Vec::new()}
        }
    }
}

impl Deck
{
    /// Deck constructor returning a Deck with 78 cards to play tarot
    pub fn new() -> Self {
        let mut deck: Deck = Deck{
            cards: Vec::new()
        };
        deck.init_tarot();
        deck
    }

    // add given card to the deck
    pub fn add_card(&mut self, card: Box<dyn Card>){
        self.cards.push(card);
    }

    /// intialize a deck with all cards needed for tarot
    ///
    /// # Examples
    ///
    /// ```
    /// use tarot_lib::card::Deck;
    /// let deck = Deck::new();
    ///
    /// assert_eq!(deck.cards.len(), 78);
    /// ```
    fn init_tarot(&mut self) {
        self.cards.clear();
        
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Un}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Deux}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Trois}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Quatre}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Cinq}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Six}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Sept}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Huit}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Neuf}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Dix}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Onze}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Douze}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Treize}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Quatorze}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Quinze}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Seize}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::DixSept}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::DixHuit}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::DixNeuf}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Vingt}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::VingtEtUn}));
        self.cards.push(Box::new(AtoutCard{value: AtoutValue::Excuse}));

        self.cards.push(Box::new(ClassicCard{value: CardValue::As, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Deux, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Trois, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Quatre, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cinq, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Six, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Sept, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Huit, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Neuf, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dix, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Valet, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cavalier, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dame, color: CardColor::Pique}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Roi, color: CardColor::Pique}));

        self.cards.push(Box::new(ClassicCard{value: CardValue::As, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Deux, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Trois, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Quatre, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cinq, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Six, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Sept, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Huit, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Neuf, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dix, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Valet, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cavalier, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dame, color: CardColor::Coeur}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Roi, color: CardColor::Coeur}));
        
        self.cards.push(Box::new(ClassicCard{value: CardValue::As, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Deux, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Trois, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Quatre, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cinq, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Six, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Sept, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Huit, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Neuf, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dix, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Valet, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cavalier, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dame, color: CardColor::Carreau}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Roi, color: CardColor::Carreau}));

        self.cards.push(Box::new(ClassicCard{value: CardValue::As, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Deux, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Trois, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Quatre, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cinq, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Six, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Sept, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Huit, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Neuf, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dix, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Valet, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Cavalier, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Dame, color: CardColor::Trefle}));
        self.cards.push(Box::new(ClassicCard{value: CardValue::Roi, color: CardColor::Trefle}));
    }

    /// distribute a deck to the given number of players
    ///
    /// # Examples
    ///
    /// ```
    /// use tarot_lib::card::Deck;
    /// let deck = Deck::new();
    /// let table = deck.distribute_to_players(4);
    /// assert_eq!(table.deck_player1.cards.len(), 18);
    /// assert_eq!(table.deck_player5.cards.len(), 0);
    /// assert_eq!(table.deck_chien.cards.len(), 6);
    /// ```
    pub fn distribute_to_players(self, nb_players: u8) -> Table {
        let mut copy_of_deck = self.cards;
        let mut table: Table = Table::new();

        let mut taille_du_chien = 6;
        let mut taille_dun_jeu = 18;
        //determination de la taille du chien
        if nb_players == 4 {
            taille_du_chien = 6;
            taille_dun_jeu = 18;
        } else if nb_players == 5 {
            taille_du_chien = 3;
            taille_dun_jeu = 16;
        }

        //tant qu'il y a des cartes a distribuer on en pioche une a distribuer
        while let Some(card) = copy_of_deck.pop() {
            //on distribue aleatoirement aux decks tant qu'ils ne sont pas pleins
            let mut rng = rand::thread_rng();
            let random = rng.gen_range(0, nb_players+1);

            if random == 1 && table.deck_player1.cards.len() < taille_dun_jeu {
                table.deck_player1.add_card(card);
            } else if random == 2 && table.deck_player2.cards.len() < taille_dun_jeu {
                table.deck_player2.add_card(card);
            } else if random == 3 && table.deck_player3.cards.len() < taille_dun_jeu {
                table.deck_player3.add_card(card);
            } else if random == 4 && table.deck_player4.cards.len() < taille_dun_jeu {
                table.deck_player4.add_card(card);
            } else if random == 5 && table.deck_player5.cards.len() < taille_dun_jeu && nb_players == 5 {
                table.deck_player5.add_card(card);
            } else if random == 0 && table.deck_chien.cards.len() < taille_du_chien {
                table.deck_chien.add_card(card);
            } else {
                //TODO ameliorer le systeme de distribution
                //le joueur/chien designe ne peut pas recevoir cette carte, on la remet dans le deck
                copy_of_deck.push(card);
            }
        }
        table
    }

    pub fn display(&self) {
        for c in self.cards.iter()
        {
            c.display();
        }
    }
}
