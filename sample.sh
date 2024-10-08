#!/bin/bash

# 引数の数を確認
if [ $# -eq 0 ]; then
  echo "ファイル名を引数として渡してください。"
  exit 1
fi

# 10秒待機
sleep 3

# ファイル名を表示
echo "ファイル名: $1"