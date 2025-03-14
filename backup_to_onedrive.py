import os
import shutil
from pathlib import Path
from datetime import datetime
from tqdm import tqdm

def get_onedrive_path():
    """Get OneDrive path from environment variables."""
    # Try common OneDrive paths
    possible_paths = [
        os.path.expandvars('%ONEDRIVE%'),
        os.path.expanduser('~/OneDrive'),
        os.path.expandvars('%USERPROFILE%/OneDrive')
    ]
    
    for path in possible_paths:
        if os.path.exists(path):
            return path
    return None

def copy_with_progress(src, dst, desc="Copying"):
    """Copy file with progress bar."""
    if os.path.exists(dst):
        return  # Skip if file exists
        
    size = os.path.getsize(src)
    with tqdm(total=size, unit='B', unit_scale=True, desc=desc) as pbar:
        shutil.copy2(src, dst)
        pbar.update(size)

def verify_copy(src_file, dst_file):
    """Verify that file was copied correctly."""
    if not os.path.exists(dst_file):
        return False
        
    try:
        src_size = os.path.getsize(src_file)
        dst_size = os.path.getsize(dst_file)
        return src_size == dst_size
    except:
        return False

def backup_directory(src_dir, dst_dir):
    """Backup directory maintaining structure."""
    try:
        # Calculate total size first
        print(f"\nCalculating size of {src_dir}...")
        total_size = sum(
            os.path.getsize(os.path.join(root, file))
            for root, _, files in os.walk(src_dir)
            for file in files
            if not any(skip in root.lower() for skip in [
                '.git', '__pycache__', 'node_modules', '.venv',
                '.vs', '.idea', '.vscode'
            ])
        )
        print(f"Total size: {total_size / (1024*1024*1024):.2f} GB")

        # Track copy status
        copied_files = []
        failed_files = []

        # Create progress bar for overall progress
        with tqdm(total=total_size, unit='B', unit_scale=True, desc="Overall Progress") as pbar:
            for root, dirs, files in os.walk(src_dir):
                # Skip certain directories
                dirs[:] = [d for d in dirs if d not in [
                    '.git', '__pycache__', 'node_modules', '.venv',
                    '.vs', '.idea', '.vscode'
                ]]

                # Create corresponding directories in destination
                rel_path = os.path.relpath(root, src_dir)
                dst_path = os.path.join(dst_dir, rel_path)
                os.makedirs(dst_path, exist_ok=True)

                # Copy files
                for file in files:
                    src_file = os.path.join(root, file)
                    dst_file = os.path.join(dst_path, file)
                    
                    # Skip certain file types and existing files
                    if any(file.endswith(ext) for ext in ['.tmp', '.log']):
                        continue

                    try:
                        size = os.path.getsize(src_file)
                        shutil.copy2(src_file, dst_file)
                        if verify_copy(src_file, dst_file):
                            copied_files.append(dst_file)
                        else:
                            failed_files.append(src_file)
                        pbar.update(size)
                    except (OSError, shutil.Error) as e:
                        print(f"\nError copying {src_file}: {str(e)}")
                        failed_files.append(src_file)

        # Print copy status
        if copied_files:
            print(f"\nSuccessfully copied {len(copied_files)} files")
        if failed_files:
            print(f"\nFailed to copy {len(failed_files)} files:")
            for file in failed_files[:10]:  # Show first 10 failed files
                print(f"  - {file}")
            if len(failed_files) > 10:
                print(f"  ... and {len(failed_files) - 10} more")

        return len(failed_files) == 0
    except Exception as e:
        print(f"\nError backing up {src_dir}: {str(e)}")
        return False

def main():
    import glob  # Add import here
    
    # Find OneDrive path
    onedrive_path = get_onedrive_path()
    if not onedrive_path:
        print("OneDrive folder not found! Please make sure OneDrive is installed and synced.")
        return
        
    print(f"Found OneDrive at: {onedrive_path}")
    
    # Create backup folder
    backup_dir = os.path.join(onedrive_path, f"WindSurf_Backup_{datetime.now().strftime('%Y%m%d_%H%M%S')}")
    os.makedirs(backup_dir, exist_ok=True)
    
    # Get user directory and app data
    user_dir = str(Path.home())
    appdata_roaming = os.path.expandvars('%APPDATA%')
    appdata_local = os.path.expandvars('%LOCALAPPDATA%')
    
    # Common code locations
    code_locations = [
        os.path.join(user_dir, 'Documents', 'GitHub'),
        os.path.join(user_dir, 'Documents', 'GitLab'),
        os.path.join(user_dir, 'Documents', 'Projects'),
        os.path.join(user_dir, 'Documents', 'Visual Studio*'),
        os.path.join(user_dir, 'source', 'repos'),
        os.path.join(user_dir, 'git'),
        os.path.join(user_dir, 'code'),
        os.path.join(user_dir, 'projects'),
        os.path.join(user_dir, 'workspace'),
        os.path.join(user_dir, 'dev'),
        os.path.join(user_dir, 'Documents', 'WebstormProjects'),
        os.path.join(user_dir, 'Documents', 'PycharmProjects'),
        os.path.join(user_dir, 'Documents', 'IntelliJProjects'),
        os.path.join(user_dir, 'Documents', 'VSCodeProjects'),
        os.path.join(user_dir, 'Documents', 'NetBeansProjects'),
        os.path.join(user_dir, 'Documents', 'EclipseWorkspace'),
        os.path.join(user_dir, 'OneDrive', 'Documents', 'GitHub'),
        os.path.join(user_dir, 'OneDrive', 'Documents', 'Projects')
    ]
    
    important_dirs = {
        'Documents': 'Documents',
        'Desktop': 'Desktop',
        'Pictures': 'Pictures',
        'Downloads': 'Downloads',
        'WindSurf': 'WindSurf Project',
        # VSCode settings and extensions
        os.path.join(appdata_roaming, 'Code', 'User'): 'VSCode/Settings',
        os.path.join(appdata_roaming, 'Code', 'extensions'): 'VSCode/Extensions',
        # PyCharm settings
        os.path.join(appdata_roaming, 'JetBrains'): 'JetBrains/Settings',
        os.path.join(user_dir, '.PyCharm*'): 'PyCharm/Settings'
    }
    
    # Add all code locations to important_dirs
    for code_path in code_locations:
        if '*' in code_path:
            # Handle wildcard paths
            base_name = os.path.basename(code_path.replace('*', ''))
            important_dirs[code_path] = f'Code/{base_name}'
        else:
            if os.path.exists(code_path):
                base_name = os.path.basename(code_path)
                important_dirs[code_path] = f'Code/{base_name}'
    
    print(f"\nStarting backup to OneDrive: {backup_dir}")
    print("\nScanning for code repositories...")
    
    # Find all git repositories
    git_repos = set()
    for dir_pattern in important_dirs:
        if '*' in str(dir_pattern):
            matching_dirs = glob.glob(str(dir_pattern))
            for dir_path in matching_dirs:
                if os.path.exists(dir_path):
                    for root, _, _ in os.walk(dir_path):
                        if os.path.exists(os.path.join(root, '.git')):
                            git_repos.add(root)
        else:
            dir_path = str(dir_pattern)
            if os.path.exists(dir_path):
                for root, _, _ in os.walk(dir_path):
                    if os.path.exists(os.path.join(root, '.git')):
                        git_repos.add(root)
    
    # Add found git repos to important_dirs
    for repo in git_repos:
        repo_name = os.path.basename(repo)
        important_dirs[repo] = f'Code/GitRepos/{repo_name}'
    
    print(f"\nFound {len(git_repos)} Git repositories")
    print("Note: Files that already exist in OneDrive will be skipped")
    
    # Backup each directory
    total_backed_up = 0
    for dir_pattern, display_name in important_dirs.items():
        # Handle wildcard patterns (for PyCharm)
        if '*' in dir_pattern:
            matching_dirs = glob.glob(dir_pattern)
            for src_path in matching_dirs:
                if os.path.exists(src_path):
                    print(f"\nBacking up {display_name} from {src_path}...")
                    dir_name = os.path.basename(src_path)
                    dst_path = os.path.join(backup_dir, display_name, dir_name)
                    if backup_directory(src_path, dst_path):
                        total_backed_up += 1
        else:
            # Normal directory backup
            if dir_pattern == 'WindSurf':
                src_path = os.path.join(os.path.dirname(os.path.abspath(__file__)))
            else:
                src_path = dir_pattern
                
            if os.path.exists(src_path):
                print(f"\nBacking up {display_name}...")
                dst_path = os.path.join(backup_dir, display_name.replace('/', os.sep))
                if backup_directory(src_path, dst_path):
                    total_backed_up += 1
    
    print(f"\nBackup completed!")
    print(f"Directories backed up: {total_backed_up}/{len(important_dirs)}")
    print(f"Backup location: {backup_dir}")
    print("\nWaiting for OneDrive to sync...")
    print("You can check the OneDrive app to monitor sync progress.")
    print("\nOnce sync is complete, you can safely install Ubuntu or NixOS!")

if __name__ == "__main__":
    main()
