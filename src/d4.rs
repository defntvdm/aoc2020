struct Passport {
    byr: Option<i64>,
    iyr: Option<i64>,
    eyr: Option<i64>,
    cid: Option<i64>,
    pid: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            pid: None,
            cid: None,
            hgt: None,
            hcl: None,
            ecl: None,
        }
    }

    fn is_valid_s1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.pid.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
    }

    fn byr_is_valid(&self) -> bool {
        if let Some(byr) = self.byr {
            if byr < 1920 || byr > 2002 {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn iyr_is_valid(&self) -> bool {
        if let Some(iyr) = self.iyr {
            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn eyr_is_valid(&self) -> bool {
        if let Some(eyr) = self.eyr {
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn hgt_is_valid(&self) -> bool {
        if let Some(hgt) = &self.hgt {
            if hgt.len() < 3 {
                return false;
            }
            match &hgt[hgt.len() - 2..] {
                "cm" => {
                    let h = hgt[..hgt.len() - 2].parse::<i64>();
                    if let Ok(he) = h {
                        if he < 150 || he > 193 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                "in" => {
                    let h = hgt[..hgt.len() - 2].parse::<i64>();
                    if let Ok(he) = h {
                        if he < 59 || he > 76 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            };
        } else {
            return false;
        }

        true
    }

    fn hcl_is_valid(&self) -> bool {
        if let Some(hcl) = &self.hcl {
            let mut chrs = hcl.chars();
            if hcl.len() != 7 || chrs.nth(0).unwrap() != '#' {
                return false;
            }
            for _ in 1..hcl.len() {
                let ch = chrs.nth(0).unwrap();
                if !('0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'f') {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }

    fn ecl_is_valid(&self) -> bool {
        if let Some(ecl) = &self.ecl {
            match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return false,
            };
        } else {
            return false;
        }

        true
    }

    fn pid_is_valid(&self) -> bool {
        if let Some(pid) = &self.pid {
            if pid.len() != 9 {
                return false;
            }

            for c in pid.chars() {
                if '0' > c || '9' < c {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }

    fn is_valid_s2(&self) -> bool {
        self.byr_is_valid()
            && self.iyr_is_valid()
            && self.eyr_is_valid()
            && self.hgt_is_valid()
            && self.hcl_is_valid()
            && self.ecl_is_valid()
            && self.pid_is_valid()
    }

    fn copy(&self) -> Passport {
        Passport {
            byr: self.byr,
            iyr: self.iyr,
            eyr: self.eyr,
            cid: self.cid,
            pid: self.pid.clone(),
            hgt: self.hgt.clone(),
            hcl: self.hcl.clone(),
            ecl: self.ecl.clone(),
        }
    }
}

pub fn solve() {
    let inp = include_str!("../input/d4.txt");
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Passport::new();
    inp.lines().for_each(|line| {
        if line == "" {
            passports.push(passport.copy());
            passport = Passport::new();
        }
        line.split(' ').for_each(|field| {
            let kv: Vec<&str> = field.split(':').collect();
            match kv[0] {
                "byr" => passport.byr = Some(kv[1].parse().unwrap()),
                "iyr" => passport.iyr = Some(kv[1].parse().unwrap()),
                "eyr" => passport.eyr = Some(kv[1].parse().unwrap()),
                "cid" => passport.cid = Some(kv[1].parse().unwrap()),
                "pid" => passport.pid = Some(String::from(kv[1])),
                "hgt" => passport.hgt = Some(String::from(kv[1])),
                "hcl" => passport.hcl = Some(String::from(kv[1])),
                "ecl" => passport.ecl = Some(String::from(kv[1])),
                _ => {}
            }
        })
    });
    passports.push(passport.copy());
    let mut valid_count_s1 = 0;
    let mut valid_count_s2 = 0;
    for passport in passports.iter() {
        if passport.is_valid_s1() {
            valid_count_s1 += 1;
        }
        if passport.is_valid_s2() {
            valid_count_s2 += 1;
        }
    }
    println!("{}", valid_count_s1);
    println!("{}", valid_count_s2);
}
