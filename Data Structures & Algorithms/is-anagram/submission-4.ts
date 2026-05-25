class Solution {
    /**
     * @param {string} s
     * @param {string} t
     * @return {boolean}
     */
    isAnagram(s: string, t: string): boolean {
        if (s.length !== t.length) {
            return false;
        }

        let s_sort: string = s.split('').sort().join('');
        let t_sort: string = t.split('').sort().join('');

        return s_sort == t_sort;
    }
}
