require 'os'

target = "talkingclock"
tp = "target/release/talkingclock"

if OS.windows? then
    target = "talkingclock.exe"
    tp = "target\\release\\talkingclock.exe"
end

task :default do
    sh "cargo build --release"
end

task :test do
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

task :clean do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "cargo clean"
end

task :cleanlock do
    sh "cargo clean"
end
