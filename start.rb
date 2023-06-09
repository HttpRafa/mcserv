require 'net/http'
require 'json'

$server_urls = {
#    custom: "https://kienitz.link/host/minecraft/installer/", 
    paperMC: "https://api.papermc.io/v2/projects/" 
}
$software_types = {
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

    attr_reader :java_bin, :restart_time, :jvm_args, :server_args

    def initialize(java_bin, restart_time, jvm_args, server_args)
        @java_bin = java_bin
        @restart_time = restart_time
        @jvm_args = jvm_args
        @server_args = server_args
    end

    def to_json(*args)
        {
          java_bin: @java_bin,
          restart_time: @restart_time,
          jvm_args: @jvm_args,
          server_args: @server_args
        }.to_json(*args)
    end

end

def request_number
    found = false
    until found do
        puts "[INPUT] Please enter a number"
        value = gets.chomp
        if value.to_i.to_s == value
            return value.to_i
        end
        puts "[INPUT] Failed to parse number. Please try again"
    end
end

def request_answer(values)
    while true do
        $i = 0
        values.each {
          |item|
            puts "#{$i} | #{item}"
            $i += 1
        }
        puts "[INPUT] Please enter a number from 0 - #{values.size - 1}"
        value = gets.chomp
        if value.to_i.to_s == value
            value = value.to_i
            if value < values.size
                return values[value]
            end
        end
        puts "[INPUT] Failed to find entry. Please try again"
    end
end

def request_software
    software = request_answer $software_types.keys
    puts "-------------- [ #{software} ] --------------"
    software
end

def request_software_version(provider, software)
    versions = []
    puts "[NETWORK] Fetching versions..."
    case provider
    when "custom"
        url = URI.parse("#{$server_urls[provider.to_sym]}#{software}/versions.json")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess)
            json_data = JSON.parse(response.body)
            json_data.each { |version|
                versions.append(version)
            }
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    when "paperMC"
        url = URI.parse("#{$server_urls[provider.to_sym]}#{software}")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess)
            json_data = JSON.parse(response.body)
            json_data["versions"].each { |version|
                versions.append(version)
            }
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    else
        puts "Unknown provider: #{provider}"
    end
    version = request_answer versions
    puts "-------------- [ #{version} ] --------------"
    version
end

def complete_version(provider, software, version)
    builds = {}
    puts "[NETWORK] Fetching builds..."
    case provider
    when "custom"
        url = URI.parse("#{$server_urls[provider.to_sym]}#{software}/#{version}/builds.json")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess)
            json_data = JSON.parse(response.body)
            json_data.each { |build|
                builds[build["build"]] = build["file"]
            }
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    when "paperMC"
        url = URI.parse("#{$server_urls[provider.to_sym]}#{software}/versions/#{version}/builds")
        response = Net::HTTP.get_response(url)
        if response.is_a?(Net::HTTPSuccess)
            json_data = JSON.parse(response.body)
            json_data["builds"].each { |build|
                builds[build["build"]] = build["downloads"]["application"]["name"]
            }
        else
            puts "Error: #{response.code} - #{response.message}"
        end
    else
        puts "Unknown provider: #{version.provider}"
    end
    max_build = builds.keys[0]
    max_file = builds.values[0]
    builds.each {
        | key, _|
            if key > max_build
                max_build = key
                max_file = builds[key]
            end
    }
    puts "[VERSION] Latest build is #{max_build}"
    Version.new(provider, software, version, max_build, max_file)
end

def check_eula
    if File.exists?("installation.json") && File.read("eula.txt").include?("eula=true")
        puts "[EULA] Accepted"
    else
        puts "-------------- [ eula ] --------------"
        puts "By accepting this EULA, you acknowledge that failure to comply with the terms may result in the termination of your access to Minecraft."
        answer = request_answer %w[yes no]
        if answer == "yes"
            File.write("eula.txt", "eula=true")
            puts "[EULA] Accepted"
        end
    end
end

def load_settings
    if !File.exists?("settings.json")
        puts "[SCRIPT] No settings found"
        puts "[SETTINGS] Please enter the amount of RAM(in MB) the server should have"
        ram = request_number # Not further checks if the user enters -1 or something very big how cares
        puts "[SETTINGS] The server now starts with #{ram}MB of RAM"
        settings = ServerSettings.new("java", 10, "-Xmx#{ram}M -Xms#{ram}M", "nogui")
        json_data = settings.to_json
        File.open("settings.json", "w") do |file|
            file.write(json_data)
        end
        settings
    else
        puts "[SCRIPT] Loading current installation"
        json_data = JSON.parse(File.read("settings.json"))
        ServerSettings.new(json_data["java_bin"], json_data["restart_time"], json_data["jvm_args"], json_data["server_args"])
    end
end

def check_installation
    if !File.exists?("installation.json")
        puts "[SCRIPT] Script is running for the first time"
        nil
    else
        puts "[SCRIPT] Loading current installation"
        json_data = JSON.parse(File.read("installation.json"))
        Version.new(json_data["provider"], json_data["software"], json_data["version"], json_data["build"], json_data["file"])
    end
end

def check_for_updates(version)
    latest_version = complete_version version.provider, version.software, version.version
    if latest_version.build > version.build
        latest_version
    else
        nil
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
        url = URI.parse("#{$server_urls[version.provider.to_sym]}#{version.software}/#{version.version}/#{version.build}/#{version.file}")
        File.write(version.file, Net::HTTP.get(url))
    when "paperMC"
        url = URI.parse("#{$server_urls[version.provider.to_sym]}#{version.software}/versions/#{version.version}/builds/#{version.build}/downloads/#{version.file}")
        File.write(version.file, Net::HTTP.get(url))
    else
        puts "Unknown provider: #{version.provider}"
    end
end

def update(old_version, new_version)
    puts "[VERSION] Update found. Current version: #{old_version.build} | Latest version: #{new_version.build}"
    File.delete(old_version.file)

    # Check eula
    check_eula

    # Install version
    download_version new_version

    # Write version information
    write_version new_version
end

# Main
server_settings = load_settings

installation = check_installation
if !installation.nil?
    latest_version = check_for_updates installation
    if !latest_version.nil?
        update installation, latest_version
        installation = latest_version
    else
        puts "[VERSION] Up to date"

        # Check eula
        check_eula
    end
else
    puts "-------------- [ software ] --------------"
    software = request_software
    provider = $software_types[software]
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
    command = "#{server_settings.java_bin} #{server_settings.jvm_args} -jar #{installation.file} #{server_settings.server_args}"
    puts "[EXECUTE] #{command}"
    system(command)
    puts "-------------- [ stopped ] --------------"

    # Wait for restart_time seconds
    server_settings.restart_time.downto(1) do |x|
        puts "[RESTART] Waiting for #{x} seconds"
        sleep(1)
    end

    # Check for updates
    latest_version = check_for_updates installation
    if !latest_version.nil?
        update installation, latest_version
        installation = latest_version
    else
        puts "[VERSION] Up to date"
    end
end
