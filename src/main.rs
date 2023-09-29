use std::{cmp, env};

fn main() {
	//default inputs

	let mut display_summary = true;
	
	let mut max_iter: u64 = 30;

	let mut max_tuning_limit = 0;//0 > means disable tuning limit
	
	let mut target_cents = 0.0;
	let mut search_cent_radius = 40.0;//maybe abs value when collected as argument
	
	let args: Vec<String> = env::args().skip(1).collect();
	
	if args.is_empty() || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
		println!(
			"\
help:
 -h --help               |        flag        | display this help message
 -n --no-summary         |        flag        | do not display summary at the end
 -m --max-iter           | u64 | default = 30 | max numerator and denominator
 -l --limit              | u64 | default = 0  | tuning limit filter, disabled by default
 -t --target             | f64 | default = 0  | target interval in cents
 -r --search-cent-radius | f64 | default = 40 | search radius around target in cents
 -d --decimal            | f64 |              | alternative target interval input, as decimal
 -f --fraction           | u64/u64            | alternative target interval input, as fraction

u64 is a positive integer number 
f64 is integer or decimal number"
		);
		return;
	}
	
	
	if args.contains(&"-n".to_string()) || args.contains(&"--no-summary".to_string()) {
		display_summary = false;
	}
	
	if args.contains(&"-m".to_string()) {
		let i = args.iter().position(|x| x == "-m").unwrap() + 1;
		if i < args.len() {
			max_iter = args[i].parse::<u64>().unwrap_or(max_iter);
		}
	} else if args.contains(&"--max-iter".to_string()) {
		let i = args.iter().position(|x| x == "--max-iter").unwrap() + 1;
		if i < args.len() {
			max_iter = args[i].parse::<u64>().unwrap_or(max_iter);
		}
	}
	
	if args.contains(&"-l".to_string()) {
		let i = args.iter().position(|x| x == "-l").unwrap() + 1;
		if i < args.len() {
			max_tuning_limit = args[i].parse::<u64>().unwrap_or(0);
		}
	} else if args.contains(&"--limit".to_string()) {
		let i = args.iter().position(|x| x == "--limit").unwrap() + 1;
		if i < args.len() {
			max_tuning_limit = args[i].parse::<u64>().unwrap_or(0);
		}
	}
	
	if args.contains(&"-t".to_string()) {
		let i = args.iter().position(|x| x == "-t").unwrap() + 1;
		if i < args.len() {
			target_cents = args[i].parse::<f64>().unwrap_or(target_cents);
		}
	} else if args.contains(&"--target-cents".to_string()) {
		let i = args.iter().position(|x| x == "--target-cents").unwrap() + 1;
		if i < args.len() {
			target_cents = args[i].parse::<f64>().unwrap_or(target_cents);
		}
	}
	
	if args.contains(&"-r".to_string()) {
		let i = args.iter().position(|x| x == "-r").unwrap() + 1;
		if i < args.len() {
			search_cent_radius = args[i].parse::<f64>().unwrap_or(search_cent_radius).abs();
		}
	} else if args.contains(&"--search-cent-radius".to_string()) {
		let i = args.iter().position(|x| x == "--search-cent-radius").unwrap() + 1;
		if i < args.len() {
			search_cent_radius = args[i].parse::<f64>().unwrap_or(search_cent_radius).abs();
		}
	}
	
	if args.contains(&"-d".to_string()) {
		let i = args.iter().position(|x| x == "-d").unwrap() + 1;
		if i < args.len() {
			target_cents = decimal_to_cents(args[i].parse::<f64>().unwrap_or(1.0));
		}
	} else if args.contains(&"--decimal".to_string()) {
		let i = args.iter().position(|x| x == "--decimal").unwrap() + 1;
		if i < args.len() {
			target_cents = decimal_to_cents(args[i].parse::<f64>().unwrap_or(1.0));
		}
	}
	
	if args.contains(&"-f".to_string()) {
		let i = args.iter().position(|x| x == "-f").unwrap() + 1;
		if i < args.len() {
			target_cents = Interval::from_str(&args[i]).cents();
		}
	} else if args.contains(&"--fraction".to_string()) {
		let i = args.iter().position(|x| x == "--fraction").unwrap() + 1;
		if i < args.len() {
			target_cents = Interval::from_str(&args[i]).cents();
		}
	}
	
	//----------------------------------
	
	if max_tuning_limit <= 0 {
		max_tuning_limit = max_iter + 1;
	}
	
	let tuning_limit_filter_on = max_tuning_limit < max_iter;
	
	let mut count = 0;
	let mut smallest_tuning_limit = max_tuning_limit + 1;
	let mut smallest_cent_deviation = search_cent_radius + 1.0;
	let mut smallest_cent_deviation_interval = Interval::new(0,0);
	
	let den_div = ((target_cents + search_cent_radius) / 1200.0f64).exp2();
	let den_div2 = ((target_cents - search_cent_radius) / 1200.0f64).exp2();
	
	let mut primes = PrimeFactors::new();//reusable memory allocation
	
	for num in 1 .. max_iter + 1 {
		if tuning_limit_filter_on {//check numerator and denominator separately
			if !num_is_tuning_limit(num,max_tuning_limit) {
				continue;
			}
		}
		let mut d = cmp::max(1,((num as f64)/den_div).ceil() as u64);
		let d2 = cmp::max(1,((num as f64)/den_div2).floor() as u64);
		
		if d == 1 {
			if ((num as f64).log2() * 1200.0 - target_cents).abs() - search_cent_radius > 0.0 {
				d = 2;
			}
		}
		let reduceable_limit = cmp::min(max_tuning_limit,num);
		//let reduceable_limit = cmp::min(max_tuning_limit, (num as f64).sqrt().floor() as u64);
		for den in d .. cmp::min(max_iter, d2) + 1 {
			
			//figure out how to maybe make this outside the den loop.
			if den != 1 && num % den == 0 {
				continue;
			}
			if num != 1 && den % num == 0 {
				continue;
			}
		
			let interval = Interval::new(num,den);
			
			if tuning_limit_filter_on {
				if !num_is_tuning_limit(den,max_tuning_limit) {
					continue;
				}
			}
			let reduceable_limit = cmp::min(max_tuning_limit,den);
			if interval.is_reduceable_limit_optimized(reduceable_limit) {
				continue;
			}
			
			let cents = interval.cents();
			let cents_deviation = cents - target_cents;
			
			let mut cents_deviation_s = "".to_string();
			if cents_deviation >= 0.0 {
				cents_deviation_s += "+";
			}
			cents_deviation_s += &cents_deviation.to_string();
			
			if smallest_cent_deviation > cents_deviation.abs() {
				smallest_cent_deviation = cents_deviation.abs();
				smallest_cent_deviation_interval = interval.clone();
			}
			
			let tuning_limit = interval.tuning_limit();
			if  smallest_tuning_limit > tuning_limit {
				smallest_tuning_limit = tuning_limit
			}
			
			count += 1;
			
			println!(
				"{} | limit: {} | cents off: {} | prime factors: {}",
				interval.simple_string(),
				tuning_limit,
				format!("{:<19}", cents_deviation_s),
				interval.prime_factor_string(&mut primes)
			);
		}
	}
	
	if display_summary {
		let mut smallest_tuning_limit_s = "N/A".to_string();
		if smallest_tuning_limit <= max_tuning_limit {
			smallest_tuning_limit_s = smallest_tuning_limit.to_string();
		}
		
		println!(
			"\
----------------------------
Total intervals found: {}
Max numerator and denominator: {}
Max tuning limit: {}
Lowest tuning limit found: {}
Target cents: {}
Search cent radius: {}
Lowest cent deviation: {}  ({})
----------------------------",
			count,
			max_iter,
			max_tuning_limit,
			smallest_tuning_limit_s,
			target_cents,
			search_cent_radius,
			smallest_cent_deviation,
			smallest_cent_deviation_interval.simple_string()
		);
	}
}

fn num_is_tuning_limit(mut n: u64, limit: u64) -> bool {
	let limit = cmp::min(limit, n);
	let mut factor = 2;
	while factor <= limit {
		if n % factor == 0 {
			n /= factor;
		} else {
			factor += 1;
		}
	}
	n == 1
}

fn decimal_to_cents(n: f64) -> f64 {
	1200.0 * n.log2()
}

struct PrimeFactors {
	len: u8,
	ar: [IntPower; 16]
}

impl PrimeFactors {
	fn new() -> Self {
		Self{len: 0, ar: [IntPower::new(0,0); 16]}
	}
	
	fn set(&mut self, mut num: u64) {
		self.len = 1;
		self.ar[0].num = 2;
		self.ar[0].pow = 0;
		
		while self.ar[(self.len - 1) as usize].num <= num {
			if num % self.ar[(self.len - 1) as usize].num == 0 {
				num /= self.ar[(self.len - 1) as usize].num;
				self.ar[(self.len - 1) as usize].pow += 1;
				
			} else if self.ar[(self.len - 1) as usize].pow > 0 {
				self.ar[self.len as usize].num = self.ar[(self.len - 1) as usize].num + 1;
				self.ar[self.len as usize].pow = 0;
				self.len += 1;
				
			} else {
				self.ar[(self.len - 1) as usize].num += 1;
			}
		}
		if self.ar[(self.len - 1) as usize].pow == 0 {
			self.len -= 1;
		}
	}
	
	fn to_string(&self) -> String {
		if self.len == 0 {
			return "1".to_string();
		}
		
		let mut s = self.ar[0].to_string();
		
		for c in 1..self.len {
			s += "*";
			s += &self.ar[c as usize].to_string();
		}
		
		s
	}
}

#[derive(Copy,Clone)]
struct IntPower {
	num: u64,
	pow: u8//u64 numbers can only have max 64 pow
}

impl IntPower {
	fn new(num: u64, pow: u8) -> Self {
		Self{num, pow}
	}
	
	fn to_string(&self) -> String {
		let mut s = self.num.to_string();
		if self.pow > 1 {
			s += "^";
			s += &self.pow.to_string();
		}
		s
	}
}

#[derive(Clone)]
struct Interval {
	num: u64,
	den: u64
}

impl Interval {
	
	fn new(num: u64, den: u64) -> Self {
		Interval{ num, den }
	}
	
	fn is_reduceable_limit_optimized(&self, limit: u64) -> bool {
		self.is_reduceable_limit_precalc_optimized(cmp::min(cmp::min(self.num, self.den), limit))
	}
	
	fn is_reduceable_limit_precalc_optimized(&self, limit: u64) -> bool {
		let mut factor = 2;
		while factor <= limit {
			if self.num % factor == 0 && self.den % factor == 0 {
				return true;
			} else {
				factor += 1;
			}
		}
		false
	}
	
	fn simple_string(&self) -> String {
		self.num.to_string() + " / " + &self.den.to_string()
	}
	
	fn prime_factor_string(&self, primes: &mut PrimeFactors) -> String {
		primes.set(self.num);
		let mut s = primes.to_string();
		s += " / ";
		primes.set(self.den);
		s += &primes.to_string();
		s
	}
	
	fn tuning_limit(&self) -> u64 {
		let mut n = self.den;
		
		let mut limit = 1;
		let mut factor = 2;
		
		while 1 < n {
			if n % factor == 0 {
				n /= factor;
				limit = factor
			} else {
				factor += 1;
			}
		}
		
		
		n = self.num;
		
		let mut limit2 = 1;
		let mut factor = 2;
		
		while 1 < n {
			if n % factor == 0 {
				n /= factor;
				limit2 = factor
			} else {
				factor += 1;
			}
		}
		
		if limit2 > limit {
			return limit2
		}
		
		limit
	}
	
	fn to_f64(&self) -> f64 {
		(self.num as f64) / (self.den as f64)
	}
	
	fn cents(&self) -> f64 {
		1200.0 * self.to_f64().log2()
	}
	
	fn from_str(s: &str) -> Self {
		let pivot = s.chars().position(|c| c == '/').unwrap_or(0);
		let num = s[..pivot].parse::<u64>().unwrap_or(1);
		let den = s[pivot+1..].parse::<u64>().unwrap_or(1);
		Self::new(num,den)
	}
	
	/*
	fn new(num: u64, den: u64) -> Self {
		let mut interval = Interval{ num, den };
		interval.reduce();
		interval
	}
	*/
	
	//fn is_reduceable(&self) -> bool {
	//	self.is_reduceable_limit_precalc_optimized(cmp::min(self.num, self.den))
	//}
	
	//fn sum_num_den(&self) -> u64 {
	//	self.num + self.den
	//}
	
	/*
	fn is_tuning_limit(&self, limit: u64) -> bool {
		let mut n = self.den;
		let mut factor = 2;
		
		while factor <= limit {
			if n % factor == 0 {
				n /= factor;
			} else {
				factor += 1;
			}
		}
		
		if n > 1 {
			return false;
		}
		
		n = self.num;
		factor = 2;
		
		while factor <= limit {
			if n % factor == 0 {
				n /= factor;
			} else {
				factor += 1;
			}
		}
		
		n == 1
	}
	*/
	
	/*
	fn reduce(&mut self) {
		let mut factor = 2;
		while factor <= self.den {
			if self.num % factor == 0 && self.den % factor == 0 {
				self.num /= factor;
				self.den /= factor;
			} else {
				factor += 1;
			}
		}
	}
	*/
}

