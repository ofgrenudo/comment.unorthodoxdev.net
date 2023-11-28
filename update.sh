#!/bin/bash
printf "=======================================================\n"
printf "||                                                   ||\n"
printf "||                                                   ||\n"
printf "||do not make commits on prod or else you will break ||\n"
printf "||      the damn database. Remember 11.28.2023       ||\n"
printf "||                                                   ||\n"
printf "||                                                   ||\n"
printf "=======================================================\n\n"
printf "Backing up the comments database to \`backups\\\` \n\n"

cp comments.db backup/comments_`date +"%Y%m%d_%H%M"`.db  
printf "comments.db has been susscessfully backed up.\n"

