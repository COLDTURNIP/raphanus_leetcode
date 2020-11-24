/*
Problem 638. Shopping Offers
============================

https://leetcode.com/problems/shopping-offers/

In LeetCode Store, there are some kinds of items to sell. Each item has a price.

However, there are some special offers, and a special offer consists of one or more different kinds
of items with a sale price.

You are given the each item's price, a set of special offers, and the number we need to buy for
each item. The job is to output the lowest price you have to pay for exactly certain items as
given, where you could make optimal use of the special offers.

Each special offer is represented in the form of an array, the last number represents the price you
need to pay for this special offer, other numbers represents how many specific items you could get
if you buy this offer.

You could use any of special offers as many times as you want.

Example 1:
    Input: [2,5], [[3,0,5],[1,2,10]], [3,2]
    Output: 14
    Explanation:
    - There are two kinds of items, A and B. Their prices are $2 and $5 respectively.
    - In special offer 1, you can pay $5 for 3A and 0B
    - In special offer 2, you can pay $10 for 1A and 2B.
    - You need to buy 3A and 2B, so you may pay $10 for 1A and 2B (special offer #2), and $4 for 2A.

Example 2:
    Input: [2,3,4], [[1,1,0,4],[2,2,1,9]], [1,2,1]
    Output: 11
    Explanation:
    - The price of A is $2, and $3 for B, $4 for C.
    - You may pay $4 for 1A and 1B, and $9 for 2A ,2B and 1C.
    - You need to buy 1A ,2B and 1C, so you may pay $4 for 1A and 1B (special offer #1), and $3 for
      1B, $4 for 1C.
    - You cannot add more items, though only $9 for 2A ,2B and 1C.

Note:
    - There are at most 6 kinds of items, 100 special offers.
    - For each item, you need to buy at most 6 of them.
    - You are not allowed to buy more items than you want, even if that would lower the overall price.
*/

impl Solution {
    fn shopping_offers_impl(
        size: usize,
        special_discount_list: &[Vec<i32>],
        idx: usize,
        rest: &mut [i32],
        discount: i32,
    ) -> i32 {
        let special_discount = if let Some(sp) = special_discount_list.get(idx) {
            sp
        } else {
            return discount;
        };
        let sp_num = &special_discount[..size];
        let sp_discount = special_discount[size];
        let can_take_discount = |r: &[i32]| sp_num.iter().zip(r.iter()).all(|(&n, &r)| r >= n);
        let mut add_discount =
            Self::shopping_offers_impl(size, special_discount_list, idx + 1, rest, discount);
        for times in 1.. {
            if can_take_discount(rest) {
                for (entry, &n) in rest.iter_mut().zip(sp_num.iter()) {
                    *entry -= n;
                }
                add_discount = add_discount.max(Self::shopping_offers_impl(
                    size,
                    special_discount_list,
                    idx + 1,
                    rest,
                    discount + sp_discount * times,
                ));
            } else {
                for (entry, &n) in rest.iter_mut().zip(sp_num.iter()) {
                    *entry += n * (times - 1);
                }
                break;
            }
        }
        add_discount
    }

    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let size = price.len();
        let mut needs = needs;
        let mut special_discount_list = Vec::new();
        for mut s in special {
            let sp = s[size];
            let discount = price
                .iter()
                .zip(s.iter().take(size))
                .fold(0, |s, (&p, &n)| s + p * n)
                - sp;
            if discount > 0 {
                s[size] = discount;
                special_discount_list.push(s);
            }
        }
        let cost = price
            .iter()
            .zip(needs.iter())
            .fold(0, |s, (&p, &n)| s + p * n);
        cost - Self::shopping_offers_impl(price.len(), &special_discount_list, 0, &mut needs, 0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]),
            14
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::shopping_offers(
                vec![2, 3, 4],
                vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
                vec![1, 2, 1]
            ),
            11
        );
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::shopping_offers(
                vec![2, 3, 4],
                vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
                vec![1, 2, 1],
            )
        });
    }
}
