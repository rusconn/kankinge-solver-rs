# kankinge-solver-rs

[監禁ゲー(NSFW)](https://www.dlsite.com/maniax/work/=/product_id/RJ312391.html)をプログラムで解く試み。

## 実行方法

```sh
mise run exec <stage_file> <bfs|iddfs>
```

## メモ

- もともと[TSだった](https://github.com/rusconn/kankinge-solver)が、Rustへ移植した
  - 5~9倍程度高速化した
- BFSは支配関係による枝刈りにより浅い解(せいぜい30~50程度)なら発見できるくらいにはなっている
  - しかし空間が$`\Theta(b^d)`$で成長するのですぐに枯渇する
  - 局所的な探索に使えなくもない
- IDDFSは効果的な枝刈りができず、遅くて使えない
- 本格的な探索のためにヒューリスティックなアルゴリズムを追加する予定
- 複数タイトルへ対応できるよう移行する予定
