.PHONY: install package publish clean

package:
	npm install
	npx @vscode/vsce package

install: package
	code --install-extension harana-fml-*.vsix

publish:
	npm install
	npx @vscode/vsce publish

clean:
	rm -f *.vsix
	rm -rf node_modules
