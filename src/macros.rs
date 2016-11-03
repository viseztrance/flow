/**
 * Flow - Realtime log analyzer
 * Copyright (C) 2016 Daniel Mircea
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

macro_rules! running {
    () => (RUNNING.load(Ordering::Relaxed));
    ($val: expr) => (RUNNING.store($val, Ordering::Relaxed));
}

macro_rules! quit {
    ($msg: expr) => {
        println!("{}", $msg);
        process::exit(0)
    };
}

macro_rules! critical_quit {
    ($msg: expr) => {
        println!("{}", $msg);
        process::exit(1)
    };
}

macro_rules! assert_quit {
    ($code: expr, $msg: expr) => {
        if !$code {
            println!("{}", $msg);
            process::exit(2)
        }
    }
}
