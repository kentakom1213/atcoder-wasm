//! セグメント木（関数を渡す）

use std::fmt::{self, Debug};
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, Index, RangeBounds,
};

/// SegmentTree (Monoid)
/// - セグメント木
pub struct SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    pub size: usize,
    offset: usize,
    e: T,
    op: F,
    data: Vec<T>,
}

impl<T, F> Index<usize> for SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[self.offset + idx]
    }
}

impl<T, F> SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        }
        .min(self.size);
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        }
        .min(self.size);
        if start <= end {
            Some((start, end))
        } else {
            None
        }
    }

    /// セグメント木を初期化する
    pub fn new(n: usize, e: T, op: F) -> Self {
        let offset = n.next_power_of_two();

        Self {
            size: n,
            offset,
            e: e.clone(),
            op,
            data: vec![e; offset << 1],
        }
    }

    /// セグメント木を配列から初期化する
    pub fn build(src: &[T], e: T, op: F) -> Self {
        let mut seg = Self::new(src.len(), e, op);
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = (seg.op)(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }

    pub fn update(&mut self, index: usize, value: T) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = (self.op)(&self.data[lch], &self.data[lch + 1]);
        }
    }

    /// 可変な参照を返す
    pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, T, F>> {
        if i < self.offset {
            let default = self.index(i).clone();
            Some(ValMut {
                segtree: self,
                idx: i,
                new_val: default,
            })
        } else {
            None
        }
    }

    /// 区間`range`の集約を行う
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T {
        let parsed = self.parse_range(range);
        if parsed.is_none() {
            return self.e.clone();
        }

        let (start, end) = parsed.unwrap();

        // 全体の値を取得
        if (start, end) == (0, self.size) {
            return self.data[1].clone();
        }

        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (self.e.clone(), self.e.clone());

        while l < r {
            if l & 1 == 1 {
                res_l = (self.op)(&res_l, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = (self.op)(&self.data[r], &res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(&res_l, &res_r)
    }
}

impl<T: Debug, F> std::fmt::Debug for SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentTree {{ [").ok();
        for i in 0..self.size {
            if i + 1 < self.size {
                write!(f, "{:?}, ", self.data[self.offset + i]).ok();
            } else {
                write!(f, "{:?}", self.data[self.offset + i]).ok();
            }
        }
        write!(f, "] }}")
    }
}

pub struct ValMut<'a, T: 'a, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    segtree: &'a mut SegmentTree<T, F>,
    idx: usize,
    new_val: T,
}

impl<T: Debug, F> fmt::Debug for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut")
            .field(&self.segtree.index(self.idx))
            .finish()
    }
}

impl<T, F> Drop for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn drop(&mut self) {
        self.segtree.update(self.idx, self.new_val.clone());
    }
}

impl<T, F> Deref for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.segtree[self.idx]
    }
}

impl<T, F> DerefMut for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

// セグ木上の2分探索
impl<T, F> SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    /// 左端を固定した2分探索
    /// - 引数`l`と関数`f`に対して，
    ///     - `f( seg.get(l..x) ) = true`
    ///     - `f( seg.get(l..x+1) ) = false`
    ///
    ///   \
    ///   を満たす`x`を返す
    ///
    /// **引数**
    /// - `f` :
    ///   - `f(e) = true`
    ///   - 任意の`i`に対して，`f( seg.get(l..i) ) = false`ならば，`f( seg.get(l..i+1) ) = false`
    pub fn max_right<C>(&self, mut l: usize, f: C) -> (T, usize)
    where
        C: Fn(T) -> bool,
    {
        assert!(f(self.e.clone()));

        if l >= self.size {
            return (self.e.clone(), self.size);
        }

        l += self.offset;
        let mut sum = self.e.clone();

        // 第1段階: 条件を満たさない区間を見つける
        'fst: loop {
            while l & 1 == 0 {
                l >>= 1;
            }

            let tmp = (self.op)(&sum, &self.data[l]);

            // 満たさない区間を発見した場合
            if !f(tmp.clone()) {
                break 'fst;
            }

            sum = tmp;
            l += 1;

            // すべての領域を見終わったら終了
            if (l & l.wrapping_neg()) == l {
                return (sum, self.size);
            }
        }

        // 第2段階: 子方向に移動しながら2分探索
        while l < self.offset {
            // 左に潜る
            l <<= 1;

            let tmp = (self.op)(&sum, &self.data[l]);

            // 左に潜っても大丈夫な場合
            if f(tmp.clone()) {
                sum = tmp;
                // 右に潜る
                l += 1;
            }
        }

        (sum, l - self.offset)
    }

    /// 右端を固定した2分探索
    /// - 引数`r`と関数`f`に対して，
    ///    - `f( seg.get(x..r) ) = true`
    ///    - `f( seg.get(x-1..r) ) = false`
    ///
    ///   \
    ///   となるような`x`を返す
    ///
    /// **引数**
    /// - `f` :
    ///   - `f(e) = true`
    ///   - 任意の`i`に対して，`f( seg.get(i..r) ) = false`ならば，`f( seg.get(i-1..r) ) = false`
    pub fn min_left<C>(&self, mut r: usize, f: C) -> (T, usize)
    where
        C: Fn(T) -> bool,
    {
        assert!(f(self.e.clone()));

        if r == 0 {
            return (self.e.clone(), 0);
        }

        r += self.offset;
        let mut sum = self.e.clone();

        // 第1段階: 条件を満たさない区間を見つける
        'fst: loop {
            r -= 1;
            while r > 1 && r & 1 == 1 {
                r >>= 1;
            }

            let tmp = (self.op)(&self.data[r], &sum);

            // 満たさない区間を発見した場合
            if !f(tmp.clone()) {
                break 'fst;
            }

            sum = tmp;

            // すべての領域を見終わったら終了
            if (r & r.wrapping_neg()) == r {
                return (sum, 0);
            }
        }

        // 第2段階: 子方向に移動しながら2分探索
        while r < self.offset {
            // 右に潜る
            r = (r << 1) + 1;

            let tmp = (self.op)(&self.data[r], &sum);

            // 右に潜っても大丈夫な場合
            if f(tmp.clone()) {
                sum = tmp;
                // 左に潜る
                r -= 1;
            }
        }

        (sum, r + 1 - self.offset)
    }
}
