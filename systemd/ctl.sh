#!/usr/bin/fish

function user_ctl -a verb
	systemctl $verb paketbote-*
end

function root_ctl -a verb
	sudo systemctl $verb paketbote-*
end

switch $argv[1]
	case start
		root_ctl start
	case stop
		root_ctl stop
	case restart
		root_ctl restart
	case status
		user_ctl status
end
