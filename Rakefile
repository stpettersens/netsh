require 'os'
require 'fileutils'

source = "target/release/netsh"
target = "/usr/bin/_netsh"

if OS.windows? then
	source = "target\\release\\netsh.exe"
	target = "C:\\Windows\\_netsh.exe"
end

task :default do
	sh "cargo build --release"
end

task :install => [:default] do
	FileUtils.copy(source, target)
end
