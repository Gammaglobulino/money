mod parse;
use parse::*;

use std::fmt::Display;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq,Debug,Clone)]
pub struct USD(i32);

#[derive(PartialEq,Debug,Clone)]
pub struct GBP(i32);

#[derive(PartialEq,Debug,Clone)]
pub struct CAD(i32);

impl FromStr for GBP{
    type Err=ParseMoneyError;
    fn from_str(s:&str) -> Result<Self,Self::Err>{
        Ok(GBP(parse_sym_money(s, '£', 2)?))
    }
}

pub trait ToUSDv<T>{
    fn to_uv(&self,_: T) -> f32;
   }
pub trait FromUSDv<T>{
    fn from_uv(&self,_:f32) ->T;
    
}


impl Display for USD{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let r= (self.0 as f32) / 100.0;
        if r < 0.{
            return write!(f,"-${:.2}",-r);    
        }
        write!(f,"${:.2}",r)
    }
}

#[derive(PartialEq,Debug)]
pub struct Transaction<T>{
    from_id:i32,
    to_id:i32,
    amount:T,
}

impl Account for Ex{
    fn id(&self)->i32{
        self.ac_id
    }
}

pub struct Ex{
    ac_id:i32,
    cad:f32,
    gbp:f32,
    
}

impl ToUSDv<GBP> for Ex{
    fn to_uv(&self,g:GBP)->f32{
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<CAD> for Ex{
    fn from_uv(&self,f:f32) ->CAD{
        CAD((f/self.cad) as i32)
    }
}

pub trait Account{
    fn id(&self)->i32;
}
pub trait Exchange<T,E>{
    fn convert(&self, _:T)->E;
}


impl <E,T,F> Exchange<F,T> for E
    where E:ToUSDv<F> + FromUSDv<T>
{

    fn convert(&self,f:F) -> T{
        self.from_uv(self.to_uv(f))
    }

}

pub trait ExchangeAccount<F,T>{
    fn exchange(&self,f_id:i32,t_id:i32,amount:F)->(Transaction<F>,Transaction<T>);
}
impl <E,F,T> ExchangeAccount<F,T> for E
where E:Exchange<F,T>+Account,F:Clone,
{
    fn exchange(&self,f_id:i32,t_id:i32,amount:F) ->(Transaction<F>,Transaction<T>){
        let ft=Transaction{from_id:f_id,to_id:self.id(),amount:amount.clone()};
        let tt=Transaction{from_id:self.id(),to_id:t_id,amount:self.convert(amount)};
        (ft,tt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_gbp_to_usd() {
        let g=GBP(200);
        let ex=Ex{ac_id:30,cad:0.7,gbp:1.3};
        let (ft,tt)=ex.exchange(20, 40,g);
        assert_eq!(ft,Transaction{from_id:20,to_id:30,amount:GBP(200)});
        assert_eq!(tt,Transaction{from_id:30,to_id:40,amount:CAD(371)});
        
        
    }
    #[test]
    fn display_usd(){
        let u=USD(230);
        assert_eq!(u.to_string(),"$2.30".to_string());
    }
    
    #[test]
    fn test_gbp_fromstr(){
        let g="£32.45".parse();
        assert_eq!(g,Ok(GBP(3245)));
    }
    
    
}
