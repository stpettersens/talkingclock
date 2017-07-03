require 'os'
require 'fileutils'

target = "talkingclock"
tp = "target/release/talkingclock"
command = "talkingclock"

if OS.windows? then
    target = "talkingclock.exe"
    tp = "target\\release\\talkingclock.exe"
    command = "talkingclock.cmd"
end

task :default do
    sh "cargo build --release"
end

task :test do
    sh "#{tp} --help"
    puts ""
    sh "#{tp}"
    puts ""
    sh "#{tp} --time 13:05"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :install do
    i = ""
    if OS.windows? then
        i = "@"
    else
        sh "echo '#!/bin/sh' >> #{command}"
    end
    sh "echo #{i}cd #{Dir.pwd} >> #{command}"
    if OS.windows? then
        sh "echo @#{target} %* >> #{command}"
    else
        sh "echo './#{target} $@'' >> #{command}"
        sh "chmod +x #{command}"
    end
    puts "\nInstall to path (Enter path):"
    STDIN.gets
    FileUtils.copy(command, $_.chomp!)
    File.delete(command)
end

task :clean do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "cargo clean"
end

task :cleanlock do
    sh "cargo clean"
end
