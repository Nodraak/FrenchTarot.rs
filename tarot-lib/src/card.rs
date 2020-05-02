
// Enum
// canPlay(table) -> bool

/// the Card interface
pub trait Card{
    /// Convert the card into human readable version
    fn to_readable_string(&self) -> String;
    /// Display the card into human readable version to stdout
    fn display(&self);
    /// indicate if a Card can be played in the Table
    fn can_play() -> bool;
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
    fn can_play() -> bool {
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
    fn can_play() -> bool {
        //TODO implement
        return true;
    }
}
