# Nyarticlesの記事管理用コマンドラインツール

## これは何？

記事投稿システムNyarticlesの記事管理用コマンドラインツール。

Nyarticlesの記事は専用のプレビュー付きMarkdownエディタで管理する予定。
しかしその開発には時間がかかりそうなので、
それまではVimで記事を書き、コマンドラインツールで管理する。

## システム全体の構成

記事データは以下のようなディレクトリ構成でプロジェクト管理する。

```
.
|- config.toml
|- draft
|  |- Q63NHLDSABFYNC7KK4KSF3QPCM
|  |  |- .git
|  .  |- article.md
|  .  \- meta.json
\- post
   |- 08a262b70630d7fb1fcf12e63c0fd51
   |  |- .git
   .  |- article.md
   .  \- meta.json
```

## 使い方

```
$ nya
Usage:
create a config file        : nya init
create a draft article      : nya new
post a draft article        : nya post draft/ARTICLE_DIRECTORY
update a posted article     : nya update post/ARTICLE_DIRECTORY
save an article to local git: nya save draft/ARTICLE_DIRECTORY
or                          : nya save post/ARTICLE_DIRECTORY
list draft articles         : nya drafts
list posted articles        : nya posts
```

### 初期化

 1. GitHubアカウントを用意
 1. Settings画面の左側メニュー一番下にあるPersonal access toknsを選択
 1. Generate new tokenボタンを押す
 1. Token descriptionはお好きな名前を（例："Nyarticles"）
 1. Select scopesはgistにチェックする
 1. Generate tokenボタンを押す
 1. 生成されたアクセストークン（背景が薄い緑になっている英数字）を取っておく

以下のコマンドを実行してアクセストークンを入力。

```
$ nya init
```

以下のconfig.tomlファイルが生成される。

```toml
[github]
access_token = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
```

### 新規記事の作成（下書き）

新規下書きIDでディレクトリが生成され、Markdownファイルとmeta.jsonファイルが配置されます。またGitリポジトリが生成されて、それらのファイルがコミットされます。

```
$ nya new
new repository = draft/KNNBCIYF5BF4RJ4HVHLTNRHE7Y
$ ls -a draft/KNNBCIYF5BF4RJ4HVHLTNRHE7Y
./  ../  .git/  article.md  meta.json
```

### 下書き記事の保存

変更がGitにコミットされます。

```
$ nya save draft/KNNBCIYF5BF4RJ4HVHLTNRHE7Y
```

### 下書き記事の投稿

GitHubにGistとして投稿されます。
下書きIDの代わりにGist IDがディレクトリ名になってdraftからpostに移動します。

```
$ nya post draft/KNNBCIYF5BF4RJ4HVHLTNRHE7Y
post/068a262b70630d7fb1fcf12e63c0fd51
```

### 投稿済み記事のローカル保存

変更がローカルのGitにコミットされます。

```
$ nya save post/068a262b70630d7fb1fcf12e63c0fd51
```

### 投稿済み記事の更新

変更がGitHubのGistにpushされます。

```
$ nya update post/068a262b70630d7fb1fcf12e63c0fd51
```

### 下書き記事の一覧表示

下書き記事のディレクトリとタイトルが一覧表示されます。

```
$ nya drafts
draft/JY6PXLOXDJBZPOQORGW4ZLUCSY
  # Qiitaを脱出するぞ！
draft/KNNBCIYF5BF4RJ4HVHLTNRHE7Y
  # Nyarticles作成計画
```

### 投稿済み記事の一覧表示

投稿済み記事のディレクトリとタイトルが一覧表示されます。

```
$ nya posts
post/068a262b70630d7fb1fcf12e63c0fd51
  # Qiitaを脱出するぞ！
post/4dd52d7e44ff30928f5143b2436a83e1
  # Nyarticles作成計画
```

## TODO

 - [ ] 初期化でアクセストークンの生成をプログラムで行いたい
 - [ ] 投稿済み記事を他の環境で編集した場合の同期(pull)

