.. Rust練習帳 documentation master file, created by
   sphinx-quickstart on Sun Mar 30 23:36:04 2025.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Rust練習帳 documentation
========================

.. 
   toctree::
   :maxdepth: 2
   :caption: Contents:

.. contents:: 目次
   :depth: 2
   :local:


プロジェクト作成
-----------------------------

.. code-block:: bash

   $ cargo new [ プロジェクト名 ]
   $ cd [ プロジェクト名 ]

.. code-block:: bash

   $ tree
   .
   ├── Cargo.toml
   ├── src
   │   └── main.rs
   └── target
      └── debug
          └── [  プロジェクト名  ]


- Cargo.toml は、プロジェクトの設定ファイル
   - 拡張子.toml は、Tom's Obvious, Minimal Language の略です
- src ディレクトリには、Rust のソースコードファイルを配置
- main.rs は、Rust プログラムのデフォルトの開始点

実行
-----------------------------

.. code-block:: bash

   $ cargo run


用語
-----------------------------

**クレート**

- Rustのパッケージ単位
- ライブラリやアプリケーションをクレートとして管理