require 'net/http'
require 'json'

$serverUrls = { 
#    custom: "https://kienitz.link/host/minecraft/installer/", 
    paperMC: "https://api.papermc.io/v2/projects/" 
}
$softwareTypes = {
#    raper: "custom", 
    paper: "paperMC", 
    velocity: "paperMC" 
}

class Version 

    attr_reader :provider, :software, :version, :build, :file

    def initialize(provider, software, version, build, file)
        @provider = provider
        @software = software
        @version = version
        @build = build
        @file = file
    end

    def to_json(*args)
        {
            provider: @provider,
            software: @software,
            version: @version,
            build: @build,
            file: @file
        }.to_json(*args)
    end

end

class ServerSettings 

    attr_reader :javaBin, :restartTime, :jvmArgs, :serverArgs

    def initialize(javaBin, restartTime, jvmArgs, serverArgs)
        @javaBin = javaBin
        @restartTime = restartTime
        @jvmArgs = jvmArgs
        @serverArgs = serverArgs
    end

    def to_json(*args)
        {
            javaBin: @javaBin,
            restartTime: @restartTime,
            jvmArgs: @jvmArgs,
            serverArgs: @serverArgs
        }.to_json(*args)
    end

end

def request_number()
    found = false
    while !found do
        puts "[INPUT] Please enter a number"
        value = gets.chomp
        if value.to_i.to_s == value then
            return value.to_i
        end
        puts "[INPUT] Failed to parse number. Please try again"
    end
end

def request_answer(values)
    found = false
    while !found do
        $i = 0
        values.each {
            | value |
                puts "#$i | #{value}"
                $i += 1
        }
        puts "[INPUT] Please enter a number from 0 - #{values.size - 1}"
        value = gets.chomp
        if value.to_i.to_s == value then
            value = value.to_i
            if value < values.size then
                found = true
                return values[value]
            end
        end
        puts "[INPUT] Failed to find entry. Please try again"
    end
end

def request_software()
    software = request_answer $softwareTypes.keys
    puts "-------------- [ #{software} ] --------------"
    return software
end

def request_software_version(provider, software)
    versions = []
    puts "[NETWORK] Fetching versions..."
    case provider
    when "custom"
        url = URI.parse("#{$serverUrls[provider.to_sym]}#{software}/versions.json")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess) then
            json_data = JSON.parse(response.body)
            for version in json_data do
                versions.append(version)
            end
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    when "paperMC"
        url = URI.parse("#{$serverUrls[provider.to_sym]}#{software}")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess) then
            json_data = JSON.parse(response.body)
            for version in json_data["versions"] do
                versions.append(version)
            end
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    end
    version = request_answer versions
    puts "-------------- [ #{version} ] --------------"
    return version
end

def complete_version(provider, software, version)
    builds = {}
    puts "[NETWORK] Fetching builds..."
    case provider
    when "custom"
        url = URI.parse("#{$serverUrls[provider.to_sym]}#{software}/#{version}/builds.json")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess) then
            json_data = JSON.parse(response.body)
            for build in json_data do
                builds[build["build"]] = build["file"]
            end
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    when "paperMC"
        url = URI.parse("#{$serverUrls[provider.to_sym]}#{software}/versions/#{version}/builds")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess) then
            json_data = JSON.parse(response.body)
            for build in json_data["builds"] do
                builds[build["build"]] = build["downloads"]["application"]["name"]
            end
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    end
    maxBuild = builds.keys[0]
    maxFile = builds.values[0]
    builds.each {
        | key, value | 
            if key > maxBuild then
                maxBuild = key
                maxFile = builds[key]
            end
    }
    puts "[VERSION] Latest build is #{maxBuild}"
    return Version.new(provider, software, version, maxBuild, maxFile)
end

def check_eula()
    if File.exists?("installation.json") && File.read("eula.txt").include?("eula=true") then
        puts "[EULA] Accepted"
    else
        puts "-------------- [ eula ] --------------"
        puts "By accepting this EULA, you acknowledge that failure to comply with the terms may result in the termination of your access to Minecraft."
        answer = request_answer ["yes", "no"]
        if answer == "yes" then
            File.write("eula.txt", "eula=true")
            puts "[EULA] Accepted"
        end
    end
end

def load_settings()
    if !File.exists?("settings.json") then
        puts "[SCRIPT] No settings found"
        puts "[SETTINGS] Please enter the amount of RAM(in MB) the server should have"
        ram = request_number # Not further checks if the user enters -1 or something very big how cares
        puts "[SETTINGS] The server now starts with #{ram}MB of RAM"
        settings = ServerSettings.new("java", 10, "-Xmx#{ram}M -Xms#{ram}M", "nogui")
        json_data = settings.to_json
        File.open("settings.json", "w") do |file|
            file.write(json_data)
        end
        return settings
    else
        puts "[SCRIPT] Loading current installation"
        json_data = JSON.parse(File.read("settings.json"))
        return ServerSettings.new(json_data["javaBin"], json_data["restartTime"], json_data["jvmArgs"], json_data["serverArgs"])
    end
end

def check_installation()
    if !File.exists?("installation.json") then
        puts "[SCRIPT] Script is running for the first time"
        return nil
    else
        puts "[SCRIPT] Loading current installation"
        json_data = JSON.parse(File.read("installation.json"))
        return Version.new(json_data["provider"], json_data["software"], json_data["version"], json_data["build"], json_data["file"])
    end
end

def check_for_updates(version)
    lastestVersion = complete_version version.provider, version.software, version.version
    if lastestVersion.build > version.build then
        return lastestVersion
    else
        return nil
    end
end

def write_version(version)
    json_data = version.to_json
    File.open("installation.json", "w") do |file|
        file.write(json_data)
    end
end

def download_version(version)
    puts "[VERSION] Downloading... Depending on your internet connection, this may take some time"
    case version.provider
    when "custom"
        url = URI.parse("#{$serverUrls[version.provider.to_sym]}#{version.software}/#{version.version}/#{version.build}/#{version.file}")
        File.write(version.file, Net::HTTP.get(url))
    when "paperMC"
        url = URI.parse("#{$serverUrls[version.provider.to_sym]}#{version.software}/versions/#{version.version}/builds/#{version.build}/downloads/#{version.file}")
        File.write(version.file, Net::HTTP.get(url))
    end
end

def update(oldVersion, newVersion)
    puts "[VERSION] Update found. Current version: #{oldVersion.build} | Lastest version: #{newVersion.build}"
    File.delete(oldVersion.file)

    # Check eula
    check_eula

    # Install version
    download_version newVersion

    # Write version information
    write_version newVersion
end

# Main
serverSettings = load_settings

installation = check_installation
if !installation.nil? then
    lastestVersion = check_for_updates installation
    if !lastestVersion.nil? then
        update installation, lastestVersion
        installation = lastestVersion
    else
        puts "[VERSION] Up to date"

        # Check eula
        check_eula
    end
else
    puts "-------------- [ software ] --------------"
    software = request_software
    provider = $softwareTypes[software]
    version = request_software_version provider, software
    installation = complete_version provider, software, version

    # Check eula
    check_eula

    # Install version
    download_version installation

    # Write version information
    write_version installation
end

puts "-------------- [ restart loop ] --------------"
running = true
while running do
    
    # Start the server
    command = "#{serverSettings.javaBin} #{serverSettings.jvmArgs} -jar #{installation.file} #{serverSettings.serverArgs}"
    puts "[EXECUTE] #{command}"
    system(command)
    puts "-------------- [ stopped ] --------------"

    # Wait for restartTime seconds
    serverSettings.restartTime.downto(1) do |x|
        puts "[RESTART] Waiting for #{x} seconds"
        sleep(1)
    end

    # Check for updates
    lastestVersion = check_for_updates installation
    if !lastestVersion.nil? then
        update installation, lastestVersion
        installation = lastestVersion
    else
        puts "[VERSION] Up to date"
    end
end