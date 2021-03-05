using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace openvpn_auth_user_pass
{
    class Program
    {
        const string PassFile = "psw.txt";
        const string PassLogFile = "psw-log.txt";

        static void Main(string[] args)
        {
            var currentTs = DateTimeOffset.Now.ToString("yyyy-MM-dd HH:mm:ss");
            var userName = Environment.GetEnvironmentVariable("username");
            var password = Environment.GetEnvironmentVariable("password");

            try
            {
                var passLines = File.ReadAllLines(PassFile);
                if (passLines.Contains($"{userName},{password}"))
                {
                    using (var writer = File.AppendText(PassLogFile))
                    {
                        writer.WriteLine($"{currentTs} Successful authentication: {userName}");
                    }

                    Environment.Exit(0);
                }
                else
                {
                    using (var writer = File.AppendText(PassLogFile))
                    {
                        writer.WriteLine($"{currentTs} Failed authentication: {userName},{password}");
                    }
                }
            }
            catch (Exception ex)
            {
                using (var writer = File.AppendText(PassLogFile))
                {
                    writer.WriteLine($"{currentTs} Failed operation: " + ex.Message);
                }
            }

            Environment.Exit(1);
        }
    }
}
