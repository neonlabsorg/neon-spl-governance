//! CONFIG MODULE

use cfg_if::cfg_if;
use const_format::formatcp;
use spl_governance::state::enums::MintMaxVoteWeightSource;

macro_rules! neon_elf_param {
    ($identifier:ident, $value:expr) => {
        #[no_mangle]
        #[used]
        #[allow(missing_docs)]
        pub static $identifier: [u8; $value.len()] = {
            #[allow(clippy::string_lit_as_bytes)]
            let bytes: &[u8] = $value.as_bytes();

            let mut array = [0; $value.len()];
            let mut i = 0;
            while i < $value.len() {
                array[i] = bytes[i];
                i += 1;
            }
            array
        };
    }
}

macro_rules! voter_weight_array {
    ($identifier:ident, [ $(($value_pubkey:expr,$value_weight:expr),)* ]) => {
        /// Voter Weight List
        #[no_mangle]
        #[used]
        pub static $identifier: [(::solana_program::pubkey::Pubkey,u64); [$(($value_pubkey,$value_weight),)*].len()] = [
            $((::solana_program::pubkey!($value_pubkey),$value_weight),)*
        ];
    };
}

/// Token multiplier (based on token precission)
pub const TOKEN_MULT: u64 = u64::pow(10, 9);

/// Extra tokens (not locked)
pub const EXTRA_TOKENS: u64 = 290_000_000 * TOKEN_MULT;

/// Supply fraction to calculate MaxVoterWeight
pub const SUPPLY_FRACTION: u64 = MintMaxVoteWeightSource::SUPPLY_FRACTION_BASE/10;

neon_elf_param!( PARAM_TOKEN_MULT      , formatcp!("{:?}", TOKEN_MULT));
neon_elf_param!( PARAM_EXTRA_TOKENS    , formatcp!("{:?}", EXTRA_TOKENS));
neon_elf_param!( PARAM_SUPPLY_FRACTION , formatcp!("{:?}", SUPPLY_FRACTION));

cfg_if! {
    if #[cfg(feature = "mainnet")] {

        voter_weight_array!(
            VOTER_LIST,
            [
                ("482nKGVFN1efNeBiCAkPrWATESj9Sxn6FSpzqNBoFFyg", 188762400 * TOKEN_MULT),
                ("Dsc7huV17uZQWW4LG7K2o3TEiGKXTZNjxkARz2xzFu1d", 60000000 * TOKEN_MULT),
                ("26kiPimzAioocLxZAmCvkPqgLtQL6xUSCMwkRvCSFc6j", 145250000 * TOKEN_MULT),
                ("2FWwpJHitWEk9nqte8M6CQSzCUxogUdUfjco8pPfXozX", 1000000 * TOKEN_MULT),
                ("6tTYuzuZN31iHdFLQCjmoxqatoWMYpFM8qfXGo89AWK1", 1250000 * TOKEN_MULT),
                ("27HjgEX8WxtmSMSogVLZJUKP3GrRN6A7zmgb7JZR3tMg", 1250000 * TOKEN_MULT),
                ("7XYeZmjzjefApSCswonsr2NsNB81YmHskPwffzBtmqrH", 1250000 * TOKEN_MULT),
                ("BU6N2Z68JPXLf247iYnHUTUv1B7p8AFWGTYkcjfeSwY8", 42500000 * TOKEN_MULT),
                ("EaKk38a3S4XKum2YM8gEX6KSaW9CE9AbbUaW5xQpoTTC", 42500000 * TOKEN_MULT),
                ("GUSDGuq94QYpj3YysYfnkgiKWeNcXanV2LgMrqFnsLBs", 53750000 * TOKEN_MULT),
                ("DEskk1zj5w8hvfMf5rSkxUZLcZf7sGrf5G49C7wNQNce", 7500000 * TOKEN_MULT),
                ("SMyuMjKsBJeHbqUerkpduW1TfwErdBLrXTLsx7BrgMm", 3750000 * TOKEN_MULT),
                ("69GA1mJCEqyYxj57CCeamy2WGx7wM3ABEwuUFMmatu2d", 40000000 * TOKEN_MULT),
                ("5CmWF9DMrcCtpuw3g1rnx9zYLX39bNwEX7dSEeaKFPPf", 40000000 * TOKEN_MULT),
                ("HFTXn5oTGo9dgSJfgCAU59caaiwLWx1ZDy7VjE1qu4w", 20000000 * TOKEN_MULT),
                ("6C3PmbTHi5xFZMW7c66xLvbQciVddbEFWJGpHVz1LGxX", 20000000 * TOKEN_MULT),
                ("FZXQwFXdHk4HaMhSKczdt3C4UseJpJiBn9hm8UHJWb8G", 4000000 * TOKEN_MULT),
                ("FYeKmwTpJGqZ2pzvSzzDAmwipT2J2AD3BiTdUdqTUbVv", 3000000 * TOKEN_MULT),
                ("GrjW2DtUd7WxVz1NYwguFpue5pHtVx6kqADjJqMNnwVD", 3000000 * TOKEN_MULT),
                ("CthYJnfjz9YELmZPYVJn2A1yhpmDTLUdWKuhYwEyCYZz", 2400000 * TOKEN_MULT),
                ("F5hTRH4Lu6fRkn6Scc5ogDdoFupz9oRM9fNHQfLRbehV", 2200000 * TOKEN_MULT),
                ("73dy4VtrmYoYwo2Q3q5soGwXhKngGrgnqvL5GEryC5Lk", 2000000 * TOKEN_MULT),
                ("5cs6vpXKuKKNbzpDgzRSbdMdxej7qF3hQ5ccg7L7HV4n", 1440000 * TOKEN_MULT),
                ("BWpZ4LwWg3ZV2fgQW6hxP1SmMMhwQkaKqtfq5xcx4zkd", 1400000 * TOKEN_MULT),
                ("GtH2jmBppV8VAtbEKAngGnn6h9esv9MRtgqKARFDFrbf", 1200000 * TOKEN_MULT),
                ("ASLWzyVKsmYWHY8gYRVxJtBYd3UYkg19jeo8Wrhpb3rf", 1000000 * TOKEN_MULT),
                ("AXV1sKb86s1PfYSJ78YMKwq4ejhjKtvYZh9RhyrEyuB6", 1000000 * TOKEN_MULT),
                ("AUzMEoeKiLQWWGcZ38M6nTMKWr8SpeYViHSQtm9LfHue", 1000000 * TOKEN_MULT),
                ("EYYPcCewaYKhEtA7NymW83En8it7PmaxDiDqVEaDPMea", 1000000 * TOKEN_MULT),
                ("6Uqh9XMvx3L4g82W1qoduZUt3DeG19PPr6FM3gjgwYAg", 1000000 * TOKEN_MULT),
                ("9RKL5qesjTz6YNRRoehvu6qz9HCVp3ToV5w4Dz5aq8Xv", 1000000 * TOKEN_MULT),
                ("GCtjNA958Nb1w3noeGHm5EZNh3pp6XyLjLB7yrP1wWRH", 1000000 * TOKEN_MULT),
                ("H9LTEpFCiM8jxaEYrFyqhLaMEtjkNTMbpMimctHoeQDo", 857600 * TOKEN_MULT),
                ("4dDgPddsnJHznoEoBxpukYT6YmF5JXZkt5tKV7FSxhfs", 840000 * TOKEN_MULT),
                ("9kbwhpRLdXrsJgMkyEmtqHEn12gNL4pfW3WmX66gAqaT", 640000 * TOKEN_MULT),
                ("AN7zkfE7MWVcSEPHqGFrMNKrKgUntxpdgVBAH6YuThCp", 600000 * TOKEN_MULT),
                ("GquqZs6x4gQgpqyRnengHHWCKowrKWwAR2RhMmvdwBPV", 600000 * TOKEN_MULT),
                ("QuLWFDsYsrnjvgkT4XKYn5p2tkr4h6UUBB2Q8QfZu9E", 600000 * TOKEN_MULT),
                ("GwF6shT6ahXhHrwRg9if3YTfLJfwXNX2ZLVFGfqe2jvH", 600000 * TOKEN_MULT),
                ("FrPa7KM25m5fqbxW7VVHBovUZdi7hp3HTemwJhHa44jg", 500000 * TOKEN_MULT),
                ("2AuvzJ8jK9RrYcanKxLoamUiffNP7Vm6JdYxWCq74WFj", 500000 * TOKEN_MULT),
                ("2jUVAfmhwN3znyKZ95RZLzGi2x7ghXgj3pZy4Aij163t", 480000 * TOKEN_MULT),
                ("BAHbicz9bMb2qEjPgHgU32M711QCRVRK4xSDKBcETs9d", 400000 * TOKEN_MULT),
                ("6cYPAViwm7XBDH6RKrReM8QkSiDrWbxUzDEa2sKmNGL1", 400000 * TOKEN_MULT),
                ("DXJgRvrkafSzRL7kVq8f23NbXLHcBEQKe9W9zZJvAUEe", 400000 * TOKEN_MULT),
                ("3ovnDC6Md3F2i8RT3MvZcS7QekmUq8jso1e3ke8MmY3a", 400000 * TOKEN_MULT),
                ("CcgxjNdLRx83Wrg2qhbgvCPCk8MhpBrevgcof15BZDmB", 400000 * TOKEN_MULT),
                ("6VzjkuiyMjipt4e3qw87mH6sHVeN8uxsY8qK8PxkZDYK", 400000 * TOKEN_MULT),
                ("2n9Rf5KJDVR4GKpW4JHGEaKLRw3z89uuWf27dLbQqPWZ", 400000 * TOKEN_MULT),
                ("8Fogg1kwSYzyZCRBbniVWeprPwg8s8yShtXKUwyjczpA", 300000 * TOKEN_MULT),
                ("EcBVA94VYUr6mAZqxGdB4QN787H5a3N3kd8tb4mCySs", 300000 * TOKEN_MULT),
                ("G8wh49cSsBQGioJKB5F6k9aXEkZMtY7Pjs1aZiSRhqMZ", 300000 * TOKEN_MULT),
                ("Fw9t5qZU7uCdQhRqcXayPbXCNNwEsR1ry4SWMBNDtVMB", 240000 * TOKEN_MULT),
                ("2Yf2eKbaAHxFCNUGPCqaqsXzqxMRb14C9JSPTKD3wDMF", 200000 * TOKEN_MULT),
                ("FK7Kw7WnJXjt2nBUwF5AH1omrBYsaxXWt9PXQSCDPfuT", 200000 * TOKEN_MULT),
                ("Dnu8pp4ttSxrS4weQ3drG5c873P97NoGoSnkvZY8AAkB", 200000 * TOKEN_MULT),
                ("6WYtgZjuHD1hPDiNtDDU7QQpWw576bxoSbQyd7WQoobq", 200000 * TOKEN_MULT),
                ("3Kr99Jqaw1VRHecKHhvb7BxNL7ZgyvafqA5tTZqUAJgK", 200000 * TOKEN_MULT),
                ("HaJfzhg3RdB9vueG2qmVW6ajjGSw3q3yVd3XF5MnELCz", 200000 * TOKEN_MULT),
                ("D5ntoe2zA7b2GnjHXeLytkW1zoaaSxieSibH7NhQvBQ7", 200000 * TOKEN_MULT),
                ("BgtCPrqwftgRy7yqAQSajd3woQK24E3RPfkfbtyB57km", 200000 * TOKEN_MULT),
                ("D9bpPfFu2xPZJdKDKV8iJLyhhKZuaucCEcsR7cVNAYjP", 200000 * TOKEN_MULT),
                ("8oGy9tu6KWcFa8SHoBEHSmMLmDdzEPw3PZxLEuybpKJ9", 200000 * TOKEN_MULT),
                ("DSiaQPpLwY73tpcKHi65MxbTmBif56qH2mYkdtUEia1i", 200000 * TOKEN_MULT),
                ("97vxiqEJQrpJZez7hGcCXNqfZ8k9qWQTMbvtDtgv8PL8", 200000 * TOKEN_MULT),
                ("5jrkc3RvjE9NYwYts1TvNkqjNeSjpQLFB7Sohq6zxWwp", 200000 * TOKEN_MULT),
                ("Ee6wuzBuzBQ1ScLvASQuYygbRBEWUgq3oTQJyWjjGhJF", 200000 * TOKEN_MULT),
                ("5euPxonYkrwyJKmRQe1gjsADb8RqccosbThZ3yVSf2x8", 160000 * TOKEN_MULT),
                ("Be2Ec9REaFYRcuHhtLN9hfsniFWe19LEmy3xFnkrSmv5", 160000 * TOKEN_MULT),
                ("B5C7P4F6NySR7BJFWTBUdr9Z1QDivFEjd8oQdmydJiNu", 100000 * TOKEN_MULT),
                ("DY4v61XYV7Tmf9YVWkxvFLUn3V3rccukz6Jewq7CgGCN", 100000 * TOKEN_MULT),
                ("CgWTnErgdNAzKoeRSQ1NHcW2B5ij8yfZkquVNqV3AByW", 80000 * TOKEN_MULT),
                ("A7uc5dBwaz4BjFHDm7582MHviSQ2Rq58tcUV2PA5n2Xo", 40000 * TOKEN_MULT),
            ]
        );

    } else {

        voter_weight_array!(
            VOTER_LIST,
            [
                ("482nKGVFN1efNeBiCAkPrWATESj9Sxn6FSpzqNBoFFyg", 188762400 * TOKEN_MULT),
                ("Dsc7huV17uZQWW4LG7K2o3TEiGKXTZNjxkARz2xzFu1d", 60000000 * TOKEN_MULT),
                ("26kiPimzAioocLxZAmCvkPqgLtQL6xUSCMwkRvCSFc6j", 149000000 * TOKEN_MULT),
                ("2FWwpJHitWEk9nqte8M6CQSzCUxogUdUfjco8pPfXozX", 1000000 * TOKEN_MULT),
                ("GUSDGuq94QYpj3YysYfnkgiKWeNcXanV2LgMrqFnsLBs", 150000000 * TOKEN_MULT),
                ("keyBcYtD2h6PTWvx8Ewwrak2w72hoM5VdBNbwNqwmuX", 40000000 * TOKEN_MULT),
                ("keyNBBcjcqbTGiEyihcS6FodYh68sPkWs6RG5yfLDCN", 40000000 * TOKEN_MULT),
                ("HFTXn5oTGo9dgSJfgCAU59caaiwLWx1ZDy7VjE1qu4w", 20000000 * TOKEN_MULT),
                ("tst18qx7Kd3ELAsM3Qxn4nKNRZeg26Zi7GKGHaeWFm6", 20000000 * TOKEN_MULT),
                ("tst6RG7t1J8XN3NYLNHkA3acfZcjurhurG7Kk3gAw9k", 4000000 * TOKEN_MULT),
                ("tst6YyNdi4nGhHAew2N9GKLfVE2gp99y4y4XNAo52qs", 3000000 * TOKEN_MULT),
                ("tstCUGzLUYcuuDVGgAzwi334fDhDS2asqHqcurDqhrS", 3000000 * TOKEN_MULT),
                ("tstD4uLc8NE7JYXgKdamx8f3JpC3usDLcbiyDpdbrxJ", 2400000 * TOKEN_MULT),
                ("tstKY6DqH9u7uwVw2qa3pgfJNoKWm12e82JRuccBwvV", 2200000 * TOKEN_MULT),
                ("tstnGPJyiQMUJqZxqvK4857xeWp7ZrczqZwsf4SB7R8", 2000000 * TOKEN_MULT),
                ("tstPSu5sHGrZQraZ3Ef8MFmeSfKWxQSwQQviv7cYWwb", 1440000 * TOKEN_MULT),
                ("11111111111111111111111111111111", 23197600 * TOKEN_MULT),

            ]
        );
    
    }
}
