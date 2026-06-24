# POS 全栈 monorepo —— 一键启停
# dev-start 一键起全部 app（server + pos-console + platform-admin，手动后台进程）。
# postgres/redis 由 ../002_infrastructure 独立常驻，本项目只连接、不启停。
.NOTPARALLEL:
.DEFAULT_GOAL := help
SHELL := /bin/bash
INFRA_DIR ?= ../002_infrastructure
API_URL   := http://localhost:8080
POS_URL   := http://localhost:5173
ADMIN_URL := http://localhost:5174

help: ## 显示可用目标
	@grep -E '^[a-zA-Z_-]+( [a-zA-Z_-]+)*:.*?## .*$$' $(MAKEFILE_LIST) \
	  | sed 's/ [a-z-]*:/:/' | awk 'BEGIN{FS=":.*?## "}{printf "  \033[36m%-12s\033[0m %s\n",$$1,$$2}'

# ===== 一键起全部（app；基建只连不启停）=====
dev-start start: check-db migrate ## 一键起全部：server + pos-console + platform-admin
	@./scripts/run.sh start server "cargo run --manifest-path server/Cargo.toml --bin pos-server"
	@./scripts/run.sh wait-http $(API_URL)/health
	@./scripts/run.sh start pos   "npm --prefix client/pos-console run dev"
	@./scripts/run.sh start admin "npm --prefix client/platform-admin run dev"
	@$(MAKE) --no-print-directory addrs

dev-stop stop: ## 停全部 app（不动基建）
	@./scripts/run.sh stop server pos admin

dev-restart restart: ## 重启全部 app
	@$(MAKE) --no-print-directory dev-stop && $(MAKE) --no-print-directory dev-start

dev-status status: ## app 进程状态 + 启动地址
	@./scripts/run.sh status server pos admin
	@$(MAKE) --no-print-directory addrs

addrs:
	@echo ""
	@echo "  ▸ API   (Rust)           $(API_URL)/health"
	@echo "  ▸ 收银台 pos-console      $(POS_URL)"
	@echo "  ▸ 管理台 platform-admin   $(ADMIN_URL)"
	@echo "    登录: admin/admin123 · cashier1/123456"
	@echo ""

# 基建可达性校验（不自动启动；未就绪给提示）
check-db:
	@docker exec infra-postgres pg_isready -U pos_app -d pos >/dev/null 2>&1 || { \
	  echo "✗ 基建未就绪。请先启动 postgres/redis：make infra-up"; exit 1; }

# ===== 基建（可选；仅当 002_infrastructure 未运行时手动用）=====
infra-up: ## 起基建 postgres+redis（委托 002_infrastructure）
	@$(MAKE) -C $(INFRA_DIR) up-postgres up-redis
infra-down: ## 停基建（谨慎：共享基建，影响其它项目）
	@$(MAKE) -C $(INFRA_DIR) down-postgres down-redis

# ===== 辅助 =====
migrate: ## 跑数据库迁移（服务端内嵌）
	@cargo run --quiet --manifest-path server/Cargo.toml --bin migrate
seed: ## 灌种子数据
	@cargo run --quiet --manifest-path server/Cargo.toml --bin seed
logs: ## 跟随所有 app 日志
	@tail -f .run/logs/*.log

.PHONY: help dev-start start dev-stop stop dev-restart restart dev-status status \
        addrs check-db infra-up infra-down migrate seed logs
