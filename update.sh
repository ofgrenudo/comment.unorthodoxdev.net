#!/bin/bash
@echo off
echo Backing up the current running comments.db
cp comments.db backup/comments_`date +"%Y%m%d_%H%M"`.db  