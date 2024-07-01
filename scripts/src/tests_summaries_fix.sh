#!/usr/bin/env bash

set -e

tests_summaries_fix() {
  echo "Converting test summaries to chinese chars"
  replace_char() {
    (cd mahjong_core &&
      find src -type f | grep '\.rs$' |
      xargs -I{} sed -i "s/\b$1\b/$2/g" {})
    echo "Replaced $1 with $2"
  }

  replace_char "1C" "一萬"
  replace_char "2C" "二萬"
  replace_char "3C" "三萬"
  replace_char "4C" "四萬"
  replace_char "5C" "五萬"
  replace_char "6C" "六萬"
  replace_char "7C" "七萬"
  replace_char "8C" "八萬"
  replace_char "9C" "九萬"
  replace_char "1D" "一筒"
  replace_char "2D" "二筒"
  replace_char "3D" "三筒"
  replace_char "4D" "四筒"
  replace_char "5D" "五筒"
  replace_char "6D" "六筒"
  replace_char "7D" "七筒"
  replace_char "8D" "八筒"
  replace_char "9D" "九筒"
  replace_char "1B" "一索"
  replace_char "2B" "二索"
  replace_char "3B" "三索"
  replace_char "4B" "四索"
  replace_char "5B" "五索"
  replace_char "6B" "六索"
  replace_char "7B" "七索"
  replace_char "8B" "八索"
  replace_char "9B" "九索"
  replace_char "wE" "東"
  replace_char "wS" "南"
  replace_char "wW" "西"
  replace_char "wN" "北"
  replace_char "dW" "白"
  replace_char "dG" "發"
  replace_char "dR" "中"
  replace_char "fB" "竹"
  replace_char "fC" "菊"
  replace_char "fP" "梅"
  replace_char "fO" "蘭"
  replace_char "sS" "春"
  replace_char "sM" "夏"
  replace_char "sA" "秋"
  replace_char "sW" "冬"

  (cd mahjong_core && cargo fmt)
}
