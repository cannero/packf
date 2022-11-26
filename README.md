# packf
Read package versions in a c# project and show if multiple versions of the same package are used.

```
--------------------
MULTIPLE for DotNet.Glob: 3.0.8, 3.0.5,
--------------------
Versioner { package_versions: {"AutoMapper": {"6.1.1"}, "DotNet.Glob": {"3.0.8", "3.0.5"}, "Microsoft.Bcl.Build": {"1.0.21"}, "Microsoft.Owin": {"3.1.0"}, "Microsoft.Owin.Host.HttpListener": {"3.1.0"}, "Microsoft.Owin.Host.SystemWeb": {"3.1.0"}, "Newtonsoft.Json": {"10.0.2"}, "Owin": {"1.0"}, "Serilog": {"2.8.0"}, "Serilog.Enrichers.Demystify": {"1.0.0-dev-00019"}, "Serilog.Extensions.Logging": {"2.0.4"}, "Serilog.Sinks.Console": {"3.1.1"}, "StyleCop.Analyzers": {"1.0.0"}} }
```

## known issues
- the ItemGroup elements must be in sequence in the csproj file. If there are other elements in between the file cannot be parsed.
- PackageReference does not support the `Remove` attribute

## misc
packf searches `*.csproj` and `packages.config` files. Files without read access are skipped.
Most of the test files are from [CsprojToVs2017](https://github.com/hvanbakel/CsprojToVs2017)
