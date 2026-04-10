# Default: Show help menu
default:
	@just help

# ============================================================================
# Help Command
# ============================================================================

help:
	@echo ""
	@echo "\033[1;36m======================================\033[0m"
	@echo "\033[1;36m       Agent Sight Commands          \033[0m"
	@echo "\033[1;36m======================================\033[0m"
	@echo ""
	@echo "\033[1;35m  Most Common Commands:\033[0m"
	@echo "  just \033[0;33mdev\033[0m                    \033[0;32mExport user messages to JSON\033[0m"
	@echo "  just \033[0;33mcli\033[0m                    \033[0;32mRun the Rust CLI binary\033[0m"
	@echo "  just \033[0;33mdocs\033[0m                   \033[0;32mStart the docs site\033[0m"
	@echo "  just \033[0;33mview\033[0m                   \033[0;32mView exported user messages\033[0m"
	@echo "  just \033[0;33mstats\033[0m                  \033[0;32mShow message statistics\033[0m"
	@echo ""
	@echo "\033[1;35m  Release:\033[0m"
	@echo "  just \033[0;33mpub-bump\033[0m               \033[0;32mBump patch version\033[0m"
	@echo "  just \033[0;33mpub\033[0m                    \033[0;32mTag and push a release\033[0m"
	@echo ""
	@echo "\033[1;35m  Development:\033[0m"
	@echo "  just \033[0;33mdev\033[0m                    \033[0;32mExport user messages to JSON\033[0m"
	@echo "  just \033[0;33mcli\033[0m                    \033[0;32mRun the Rust CLI binary\033[0m"
	@echo "  just \033[0;33mdocs\033[0m                   \033[0;32mStart the docs site\033[0m"
	@echo "  just \033[0;33mbuild-docs\033[0m             \033[0;32mBuild the docs site\033[0m"
	@echo ""
	@echo "\033[1;35m  Utilities:\033[0m"
	@echo "  just \033[0;33mview\033[0m                   \033[0;32mView exported user messages\033[0m"
	@echo "  just \033[0;33mstats\033[0m                  \033[0;32mShow message statistics\033[0m"
	@echo ""

# ============================================================================
# Development Commands
# ============================================================================
import 'justfiles/development/dev.just'
import 'justfiles/development/cli.just'
import 'justfiles/development/docs.just'

# ============================================================================
# Building Commands
# ============================================================================
import 'justfiles/building/build-docs.just'

# ============================================================================
# Utilities Commands
# ============================================================================
import 'justfiles/utilities/view.just'
import 'justfiles/utilities/stats.just'
import 'justfiles/utilities/pub-bump.just'
import 'justfiles/utilities/pub.just'
