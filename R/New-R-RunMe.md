#  View(New.R.RunMe)
``` R
function (global.library.folder = NULL, 
							quit.R = F,
							del.packages.that.exist.in.home.lib = F,
							update.all.packages = T)

{


	if(is.null(global.library.folder))
	{
		# finding the parent directoy of R (into which we will add the library directory)
		if(grepl("/", R.home(), fixed = T))
			{ R_parent_lib <- paste(head(strsplit(R.home(), "/", fixed = T)[[1]], -1), collapse = "/") }
		if(grepl("\\", R.home(), fixed = T))
			{ R_parent_lib <- paste(head(strsplit(R.home(), "\\", fixed = T)[[1]], -1), collapse = "/") }
		# if global.library.folder isn't defined, then we assume it is of the form: "C:\\Program Files\\R\\library"
		global.library.folder <- paste(R_parent_lib, "/library", sep = "")
	}	
	

	# checking that the global lib folder exists - and if not -> create it. (happens if it is the first time running New.R.RunMe)
	if(!file.exists(global.library.folder))
	{	# If global lib folder doesn't exist - create it.
		dir.create(global.library.folder)
		cat(paste("The path to the Global library (" , global.library.folder, ") Didn't exist - and was now created."),"\n")
	} else {
		cat(paste("The path to the Global library (" , global.library.folder, ") already exist. (NO need to create it)"),"\n")
	}

	flush.console()  # refresh the console so that the user will see the massage

	



	# Based on:
	# help(Startup)
	# checking if "Renviron.site" exists - and if not -> create it.
	Renviron.site.loc <- paste(R.home(), "\\etc\\Renviron.site", sep = "")
	if(!file.exists(Renviron.site.loc))
	{	# If "Renviron.site" doesn't exist (which it shouldn't be) - create it and add the global lib line to it.
		cat(paste("R_LIBS='",global.library.folder, "'\n",sep = "") ,
				file = Renviron.site.loc)				 
		cat(paste("The file:" , Renviron.site.loc, "Didn't exist - we created it and added your 'Global library link' (",global.library.folder,") to it.\n"))
	} else {
		cat(paste("The file:" , Renviron.site.loc, "existed and we could NOT add some lines!  make sure you add the following line by yourself:","\n"))
		cat(paste("R_LIBS=",global.library.folder,"\n", sep = "") )
		cat(paste("To the file:",Renviron.site.loc,"\n"))
	}



	# Setting the global lib for this session also
	.libPaths(global.library.folder)	# This makes sure you don't need to restart R so that the new Global lib settings will take effect in this session also
	# .libPaths(new="C:/PROGRA~1/R/library")
	# .libPaths()
	# This line could have also been added to:

	# /etc/Rprofile.site

	# and it would do the same thing as adding "Renviron.site" did

	cat("Your library paths are: ","\n")
	cat(.libPaths(),"\n")	
	flush.console()  # refresh the console so that the user will see the massage

	

	

	if(del.packages.that.exist.in.home.lib)
	{
		cat("We will now delete package from your Global library folder that already exist in the local-install library folder","\n")
		flush.console()  # refresh the console so that the user will see the massage
		package.to.del.from.global.lib <- 		paste( paste(global.library.folder, "/", sep = ""),
													list.files(paste(R.home(), "\\library\\", sep = "")),
													sep = "")			
		number.of.packages.we.will.delete <- sum(list.files(paste(global.library.folder, "/", sep = "")) %in% list.files(paste(R.home(), "\\library\\", sep = "")))
		deleted.packages <- unlink(package.to.del.from.global.lib , recursive = TRUE)	# delete all the packages from the "original" library folder (no need for double folders)

		cat(paste(number.of.packages.we.will.delete,"Packages where deleted."),"\n")
	}



	if(update.all.packages)
	{
		# Based on:
		# http://cran.r-project.org/bin/windows/base/rw-FAQ.html#What_0027s-the-best-way-to-upgrade_003f
		cat("We will now update all your packges \n")
		flush.console()  # refresh the console so that the user will see the massage
		update.packages(checkBuilt=TRUE, ask=FALSE)
	}
	
	# To quite R ?

	if(quit.R) quit(save = "no")

}
```
---
// > New.R.RunMe()
The path to the Global library ( /usr/lib/library ) Didn't exist - and was now created. 
Error in file(file, ifelse(append, "a", "w")) : 无法打开链结
此外: Warning messages:
1: In dir.create(global.library.folder) :
  由于'权限不够'的原因，无法建立'/usr/lib/library'文件目录
2: In file(file, ifelse(append, "a", "w")) :
 
 Show Traceback
 
 Rerun with Debug
 Error in file(file, ifelse(append, "a", "w")) : 无法打开链结 

 ### Rstudio