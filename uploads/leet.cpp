#include <bits/stdc++.h>
using namespace std;

#define int long long

vector<int> ops(200005, 0);
vector<int> pre(200005, 0);

void compute() {
    for (int i = 1; i < 200005; ++i) {
        int x = i;
        int count = 0;
        while (x > 0) {
            x /= 3;
            count++;
        }
        ops[i] = count;
        pre[i] = pre[i - 1] + ops[i];
    }
}

void solve() {
    int l, r;
    cin >> l >> r;
    
    int tops = pre[r] - pre[l - 1];
    
   
        tops += ops[l];
    
    
    cout << tops << endl;
}

int32_t main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    
    compute();
    
    int t;
    cin >> t;
    while (t--) {
        solve();
    }
    
    return 0;
}