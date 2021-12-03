#!/bin/bash
# calibre 设置
WORK_DIR=$PWD
OP_MOBI=$WORK_DIR/mobi/$(date "+%Y-%m-%d-%s").mobi
RECIPE=YueDu.recipe
PROFILE=kindle_pw
# Tiny Tiny RSS 设置
TTURL=https://pgw-ray.t123yh.xyz:3
if test -z ${TTUSER-} || test -z ${TTPWD-}
then
    echo "TTUSER and TTPWD must be set! exitting..."
    exit 1
fi

if test ! -d $(dirname $OP_MOBI)
then
    mkdir $(dirname $OP_MOBI)
fi

# ❤悦读 设置
# 用于生成分享链接，参考 recipe 文件的 share_url 。
XYDID=WTF_IS_OK
# 推送设置
KMAIL=example@kindle.cn
BCC_MAIL=backup_mail@outlook.com
API_KEY=example_api_key
DOMAIN=example.mailgun.org
SENDER="xinyuedu@$DOMAIN"

pushd $WORK_DIR
ebook-convert $RECIPE $OP_MOBI -vv --username "$TTURL;$TTUSER;$XYDID" --password=$TTPWD --output-profile kindle_pw 2>&1 | tee $OP_MOBI.log
# if [ $? -eq 0 ]; then
# 	curl -s --user "api:$API_KEY" \
# 		https://api.mailgun.net/v3/$DOMAIN/messages \
# 		-F from="Xin Yue Du <$SENDER>" \
# 		-F to=$KMAIL \
# 		-F bcc=$BCC_MAIL \
# 		-F subject="[❤ 悦读] $(date '+%Y年%m月%d日')" \
# 		-F text="心悦读，你的 Kindle 电子专刊。为了确保您的专刊推送，请将 $SENDER 加入白名单。" \
# 		-F attachment=@$OP_MOBI
# else
# 	curl -s --user 'api:$API_KEY' \
# 		https://api.mailgun.net/v3/$DOMAIN/messages \
# 		-F from='Xin Yue Du <$SENDER>' \
# 		-F to=$KMAIL \
# 		-F bcc=$BCC_MAIL \
# 		-F subject="[❤ 悦读] $(date '+%Y年 %m月 %d日 ')错误报告" \
# 		-F text="今日的电子书推送并未成功，请查看附件的错误日志。为了确保您的专刊推送，请将 $SENDER 加入白名单。" \
# 		-F attachment=@$OP_MOBI.log
# fi
# cp $OP_MOBI /var/www/xinyuedu/xyd/
popd
